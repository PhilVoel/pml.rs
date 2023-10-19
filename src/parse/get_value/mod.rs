mod arrays;

use std::{cell::RefCell, rc::Rc};
use crate::{elem::Element, errors::{ParseError as Error, ParseNumberError}};
use super::{ParseData, KeyType, illegal_char_err, is_char_reserved, WIPElement, TerminatorType, ISElem, WIPStruct};

type StdResult = Result<Element, Error>;
type WIPResult = Result<WIPElement, Error>;

#[derive(Copy, Clone)]
enum StringVarParseState {
    Start(KeyType, usize),
    Variable(KeyType),
    BeforeComma,
    AfterComma,
}
use StringVarParseState as VPS;

enum NumType {
    Signed,
    Unsigned,
    Decimal
}

#[derive(PartialEq, Clone, Copy)]
enum ForceCategory {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Bool,
    Struct,
    FString,
}
use ForceCategory::{I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, Bool, Struct, FString};

pub(super) fn string(parse_data: &mut ParseData, terminator_type: TerminatorType) -> Result<Vec<ISElem>, Error> {
    let terminators = match terminator_type {
        TerminatorType::Struct => vec![';'],
        TerminatorType::Array => vec![',', ']']
    };
    let mut string_elements = Vec::new();
    while let Some(c) = parse_data.next_non_whitespace() {
        match c {
            '"' => string_insert_literal(parse_data, &mut string_elements)?,
            '|' => string_insert_variable(parse_data, &mut string_elements)?,
            '#' => parse_data.skip_comment(),
            c if terminators.contains(&c) => return Ok(string_elements),
            c => return Err(illegal_char_err(c, parse_data))
        }
    }
    Err(Error::UnexpectedEOF)
}

fn string_insert_literal(parse_data: &mut ParseData, string_elements: &mut Vec<ISElem>) -> Result<(), Error> {
    let mut escape = false;
    let mut value = String::new();
    while let Some(c) = parse_data.next_char() {
        match c {
            c if escape => {
                escape = false;
                value.push(match c{
                    'r' => continue,
                    'n' => '\n',
                    't' => '\t',
                    c => c
                });
            }
            '\\' => escape = true,
            '"' => {
                string_elements.push(ISElem::Literal(value));
                return Ok(());
            }
            c => value.push(c)
        }
    }
    Err(Error::UnexpectedEOF)
}

fn string_insert_variable(parse_data: &mut ParseData, string_elements: &mut Vec<ISElem>) -> Result<(), Error> {
    let mut state;
    let mut value = String::new();
    let mut link = parse_data.nested_refs.first().expect("There should always be a struct.").clone();
    parse_data.try_skip_comment();
    match parse_data.next_non_whitespace() {
        Some('|') => return Ok(()),
        Some('"') => state = VPS::Start(KeyType::Quotes, 0),
        Some('.') => state = VPS::Start(KeyType::NoQuotes, 1),
        Some(c) if is_char_reserved(c) => return Err(illegal_char_err(c, parse_data)),
        Some(c) => {
            state = VPS::Variable(KeyType::NoQuotes);
            value.push(c);
        }
        None => return Err(Error::UnexpectedEOF)
    }
    while let Some(c) = parse_data.next_char() {
        match (state, c) {
            (VPS::Start(KeyType::Quotes, n), c @ '"') |
            (VPS::Start(KeyType::NoQuotes, n), c) if c.is_whitespace() => {
                string_elements.push(ISElem::Literal(parse_data.get_nested_key(n)));
                state = VPS::BeforeComma;
            }
            (VPS::Start(KeyType::NoQuotes, n), '#') => {
                parse_data.skip_comment();
                string_elements.push(ISElem::Literal(parse_data.get_nested_key(n)));
                state = VPS::BeforeComma;
            }
            (VPS::Start(KeyType::NoQuotes, n), '|') => {
                string_elements.push(ISElem::Literal(parse_data.get_nested_key(n)));
                return Ok(());
            }
            (VPS::Start(t, n), '.') if n < parse_data.num_of_nested() => state = VPS::Start(t, n+1),
            (VPS::Start(_, _), c) if is_char_reserved(c) => return Err(illegal_char_err(c, parse_data)),
            (VPS::Start(_, 0), c) => { // 0 only possible with KeyType::Quotes
                value.push(c);
                state = VPS::Variable(KeyType::Quotes);
            }
            (VPS::Start(s, n), c) => {
                if n <= parse_data.num_of_nested() {
                    link = parse_data.get_struct_ref(n);
                }
                value.push(c);
                state = VPS::Variable(s);
            }
            (VPS::Variable(KeyType::NoQuotes), '|') => {
                string_elements.push(ISElem::Variable(link, value));
                return Ok(());
            }
            (VPS::Variable(KeyType::Quotes), '"') => {
                string_elements.push(ISElem::Variable(link.clone(), value));
                state = VPS::BeforeComma;
                value = String::new();
            }
            (VPS::Variable(KeyType::NoQuotes), ',') => {
                string_elements.push(ISElem::Variable(link.clone(), value));
                state = VPS::AfterComma;
                value = String::new();
            }
            (VPS::Variable(KeyType::NoQuotes), '#') => {
                parse_data.skip_comment();
                state = VPS::BeforeComma;
                string_elements.push(ISElem::Variable(link.clone(), value));
                value = String::new();
            }
            (VPS::Variable(KeyType::NoQuotes), c) if c.is_whitespace() => {
                state = VPS::BeforeComma;
                string_elements.push(ISElem::Variable(link.clone(), value));
                value = String::new();
            }
            (VPS::Variable(_), '.') => value.push('.'),
            (VPS::Variable(_), c) if is_char_reserved(c) => return Err(illegal_char_err(c, parse_data)),
            (VPS::Variable(_), c) => value.push(c),
            (VPS::BeforeComma | VPS::AfterComma, '#') => parse_data.skip_comment(),
            (VPS::BeforeComma | VPS::AfterComma, c) if c.is_whitespace() => (),
            (VPS::BeforeComma, ',') => state = VPS::AfterComma,
            (VPS::BeforeComma | VPS::AfterComma, '|') => return Ok(()),
            (VPS::BeforeComma, c) => return Err(illegal_char_err(c, parse_data)),
            (VPS::AfterComma, '"') => state = VPS::Start(KeyType::Quotes, 0),
            (VPS::AfterComma, '.') => state = VPS::Start(KeyType::NoQuotes, 1),
            (VPS::AfterComma, c) if is_char_reserved(c) => return Err(illegal_char_err(c, parse_data)),
            (VPS::AfterComma, c) => {
                state = VPS::Variable(KeyType::NoQuotes);
                value.push(c);
        }
        }
    }
    Err(Error::UnexpectedEOF)
}

pub(super) fn bool(parse_data: &mut ParseData, terminator_type: TerminatorType) -> Result<bool, Error> {
    let terminators = match terminator_type {
        TerminatorType::Struct => vec![';'],
        TerminatorType::Array => vec![',', ']']
    };
    let next_c = match parse_data.next_char(){
        Some(c@ ('t' | 'f')) => c,
        Some(c) => return Err(illegal_char_err(c, parse_data)),
        None => return Err(Error::UnexpectedEOF)
    };
    let mut value = String::from(next_c);
    while let Some(c) = parse_data.next_char() {
        match (value.as_str(), c) {
            #[allow(clippy::unnested_or_patterns)]
            ("t", 'r') |
            ("tr", 'u') |
            ("tru", 'e') |
            ("f", 'a') |
            ("fa", 'l') |
            ("fal", 's') |
            ("fals", 'e') => value.push(c),
            ("true"|"false", '#') => parse_data.skip_comment(),
            ("true"|"false", c) if c.is_whitespace() => (),
            ("true"|"false", c) if terminators.contains(&c) => return Ok(value.as_str() == "true"),
            (_, c) => return Err(illegal_char_err(c, parse_data))
        }
    }
    Err(Error::UnexpectedEOF)
}

pub(super) fn pml_struct(parse_data: &mut ParseData, terminator_type: TerminatorType) -> Result<Rc<RefCell<WIPStruct>>, Error> {
    let terminators = match terminator_type {
        TerminatorType::Struct => vec![';'],
        TerminatorType::Array => vec![',', ']']
    };
    parse_data.next_char();
    let temp_struct = Rc::new(RefCell::new(WIPStruct::init()));
    parse_data.add_nested_ref(temp_struct.clone());

    while let Some(c) = parse_data.next_non_whitespace_peek() {
        if c == '}' {
            parse_data.next_char();
            parse_data.drop_last_nested_ref();
            parse_data.try_skip_comment();
            match parse_data.next_non_whitespace() {
                Some(c) if terminators.contains(&c) => return Ok(temp_struct),
                Some(c) => return Err(illegal_char_err(c, parse_data)),
                None => return Err(Error::UnexpectedEOF)
            }
        }
        let (key, value) = super::get_key_value_pair(parse_data)?;
        temp_struct.borrow_mut().add(key, value)?;
        parse_data.try_skip_comment();
    }
    Err(Error::UnexpectedEOF)
}

pub(super) fn number(parse_data: &mut ParseData, terminator_type: TerminatorType) -> StdResult {
    let (num_type, value) = get_number_type_and_string(parse_data, terminator_type)?;
    match num_type {
        NumType::Signed => match value.parse::<i128>() {
            Ok(num128) => {
                match i8::try_from(num128) {
                    Ok(num8) => Ok(num8.into()),
                    Err(_) => match i16::try_from(num128) {
                        Ok(num16) => Ok(num16.into()),
                        Err(_) => match i32::try_from(num128) {
                            Ok(num32) => Ok(num32.into()),
                            Err(_) => match i64::try_from(num128) {
                                Ok(num64) => Ok(num64.into()),
                                Err(_) => Ok(num128.into())
                            }
                        }
                    }
                }
            }
            Err(e) => Err(Error::ParseNumberError{
                line: parse_data.line,
                value,
                error: e.into()
            })
        }
        NumType::Unsigned => match value.parse::<u128>() {
            Ok(num128) => {
                match u8::try_from(num128) {
                    Ok(num8) => Ok(num8.into()),
                    Err(_) => match u16::try_from(num128) {
                        Ok(num16) => Ok(num16.into()),
                        Err(_) => match u32::try_from(num128) {
                            Ok(num32) => Ok(num32.into()),
                            Err(_) => match u64::try_from(num128) {
                                Ok(num64) => Ok(num64.into()),
                                Err(_) => Ok(num128.into())
                            }
                        }
                    }
                }
            }
            Err(e) => Err(Error::ParseNumberError{
                line: parse_data.line,
                value,
                error: e.into()
            })
        }
        NumType::Decimal => match value.parse::<f64>() {
            Ok(num64) => match value.parse::<f32>() {
                Ok(num32) => Ok(num32.into()),
                Err(_) => Ok(num64.into())
            }
            Err(e) => Err(Error::ParseNumberError{
                line: parse_data.line,
                value,
                error: e.into()
            })
        }
    }
}

fn get_number_type_and_string(parse_data: &mut ParseData, terminator_type: TerminatorType) -> Result<(NumType, String), Error> {
    let terminators = match terminator_type {
        TerminatorType::Struct => vec![';'],
        TerminatorType::Array => vec![',', ']']
    };
    let mut value = String::new();
    let mut allow_negative_sign = true;
    let mut allow_decimal_point = true;
    let mut num_type = NumType::Unsigned;
    while let Some(c) = parse_data.next_char() {
        match c {
            '-' if allow_negative_sign => {
                allow_negative_sign = false;
                num_type = NumType::Signed;
                value.push('-');
            }
            '.' if allow_decimal_point => {
                allow_decimal_point = false;
                allow_negative_sign = false;
                num_type = NumType::Decimal;
                value.push('.');
            }
            '#' => {
                parse_data.skip_comment();
                return match parse_data.next_non_whitespace() {
                    Some(c) if terminators.contains(&c) => Ok((num_type, value)),
                    Some(c) => Err(illegal_char_err(c, parse_data)),
                    None => Err(Error::UnexpectedEOF)
                }
            }
            c if c.is_ascii_digit() => value.push(c),
            c if c.is_whitespace() => return match parse_data.next_non_whitespace() {
                Some(c) if terminators.contains(&c) => Ok((num_type, value)),
                Some(c) => Err(illegal_char_err(c, parse_data)),
                None => Err(Error::UnexpectedEOF)
            },
            c if terminators.contains(&c) => return Ok((num_type, value)),
            c => return Err(illegal_char_err(c, parse_data))
        }
    }
    Err(Error::UnexpectedEOF)
}

pub(super) fn forced(parse_data: &mut ParseData, terminator_type: TerminatorType, key: &str) -> WIPResult {
    parse_data.next_char();
    let mut ftype_string = String::new();
    while let Some(c) = parse_data.next_char() {
        match c {
            '>' => {
                parse_data.try_skip_comment();
                let force_type = match ftype_string.trim() {
                    "i8" => I8,
                    "i16" => I16,
                    "i32" => I32,
                    "i64" => I64,
                    "i128" => I128,
                    "u8" => U8,
                    "u16" => U16,
                    "u32" => U32,
                    "u64" => U64,
                    "u128" => U128,
                    "f32" => F32,
                    "f64" => F64,
                    "b" => Bool,
                    "struct" => Struct,
                    "str" => FString,
                    t => return Err(Error::UnknownForcedType {
                        key: parse_data.get_full_struct_path() + "." + key,
                        type_name: t.to_string()
                    })
                };
                if parse_data.next_non_whitespace_peek() == Some('[') {
                    parse_data.next_char();
                    parse_data.add_nested_name(key.to_string());
                    let res = match force_type {
                        I8 => arrays::i8(parse_data)?,
                        I16 => arrays::i16(parse_data)?,
                        I32 => arrays::i32(parse_data)?,
                        I64 => arrays::i64(parse_data)?,
                        I128 => arrays::i128(parse_data)?,
                        U8 => arrays::u8(parse_data)?,
                        U16 => arrays::u16(parse_data)?,
                        U32 => arrays::u32(parse_data)?,
                        U64 => arrays::u64(parse_data)?,
                        U128 => arrays::u128(parse_data)?,
                        F32 => arrays::f32(parse_data)?,
                        F64 => arrays::f64(parse_data)?,
                        Bool => arrays::bool(parse_data)?,
                        Struct => arrays::structs(parse_data)?,
                        FString => arrays::strings(parse_data)?,
                    };
                    parse_data.drop_last_nested_name();
                    match parse_data.next_non_whitespace() {
                        Some(';') => return Ok(res),
                        Some(c) => return Err(illegal_char_err(c, parse_data)),
                        None => return Err(Error::UnexpectedEOF)
                    }
                }
                if force_type == Bool {
                    return Ok(bool(parse_data, TerminatorType::Struct)?.into());
                }
                if force_type == FString {
                    return Ok(string(parse_data, TerminatorType::Struct)?.into());
                }
                if force_type == Struct {
                    return Ok(pml_struct(parse_data, TerminatorType::Struct)?.into());
                }
                let (_, value) = get_number_type_and_string(parse_data, terminator_type)?;
                return match parse_forced_number(&value, force_type) {
                    Ok(element) => Ok(element.into()),
                    Err(error) => Err(Error::ParseNumberError {
                        error,
                        value,
                        line: parse_data.line,
                    })
                };
            }
            c if is_char_reserved(c) => return Err(illegal_char_err(c, parse_data)),
            c => ftype_string.push(c)
        }
    }
    Err(Error::UnexpectedEOF)
}

fn parse_forced_number(value: &str, force_type: ForceCategory) -> Result<Element, ParseNumberError> {
    Ok(match force_type {
        F32 => value.parse::<f32>()?.into(),
        F64 => value.parse::<f64>()?.into(),
        I8 => value.parse::<i8>()?.into(),
        I16 => value.parse::<i16>()?.into(),
        I32 => value.parse::<i32>()?.into(),
        I64 => value.parse::<i64>()?.into(),
        I128 => value.parse::<i128>()?.into(),
        U8 => value.parse::<u8>()?.into(),
        U16 => value.parse::<u16>()?.into(),
        U32 => value.parse::<u32>()?.into(),
        U64 => value.parse::<u64>()?.into(),
        U128 => value.parse::<u128>()?.into(),
        Bool|Struct|FString => unreachable!("This should have been caught before the function call.")
    })
}


mod arrays;

use std::{cell::RefCell, rc::Rc};
use crate::{Error, elem::Element, ParseNumberError};
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

#[derive(Clone, Copy)]
enum ForceCategory {
    S8,
    S16,
    S32,
    S64,
    S128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64
}
use ForceCategory::{S8, S16, S32, S64, S128, U8, U16, U32, U64, U128, F32, F64};

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
            (VPS::Variable(KeyType::NoQuotes), c) if c.is_whitespace() => {
                state = VPS::BeforeComma;
                string_elements.push(ISElem::Variable(link.clone(), value));
                value = String::new();
            }
            (VPS::Variable(_), '.') => value.push('.'),
            (VPS::Variable(_), c) if is_char_reserved(c) => return Err(illegal_char_err(c, parse_data)),
            (VPS::Variable(_), c) => value.push(c),
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
    let mut value = String::from(parse_data.next_char().expect("There should always be a current struct."));
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
            ("true"|"false", c) if c.is_whitespace() => (),
            ("true"|"false", c) if terminators.contains(&c) => return Ok(value.as_str() == "true"),
            (_, c) => return Err(illegal_char_err(c, parse_data))
        }
    }
    Err(Error::UnexpectedEOF)
}

pub(super) fn pml_struct(parse_data: &mut ParseData) -> Result<Rc<RefCell<WIPStruct>>, Error> {
    parse_data.next_char();
    let temp_struct = Rc::new(RefCell::new(WIPStruct::new()));
    parse_data.add_nested_ref(temp_struct.clone());

    while let Some(c) = parse_data.next_non_whitespace_peek() {
        if c == '}' {
            parse_data.next_char();
            return Ok(temp_struct);
        }
        let (key, value) = super::get_key_value_pair(parse_data)?;
        temp_struct.borrow_mut().add(key, value)?;
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

pub(super) fn forced(parse_data: &mut ParseData, terminator_type: TerminatorType, key: &str) -> StdResult {
    parse_data.next_char();
    let mut ftype_string = String::new();
    while let Some(c) = parse_data.next_char() {
        match c {
            '>' => {
                let force_type = match ftype_string.trim() {
                    "s8" => S8,
                    "s16" => S16,
                    "s32" => S32,
                    "s64" => S64,
                    "s128" => S128,
                    "u8" => U8,
                    "u16" => U16,
                    "u32" => U32,
                    "u64" => U64,
                    "u128" => U128,
                    "f32" => F32,
                    "f64" => F64,
                    t => return Err(Error::UnknownForcedType {
                        key: parse_data.get_full_struct_path() + "." + key,
                        type_name: t.to_string()
                    })
                };
                if parse_data.next_non_whitespace_peek() == Some('[') {
                    return get_forced_array(parse_data, force_type);
                }
                let (_, value) = get_number_type_and_string(parse_data, terminator_type)?;
                return match parse_forced(&value, force_type) {
                    Ok(element) => Ok(element),
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

fn parse_forced(value: &str, force_type: ForceCategory) -> Result<Element, ParseNumberError> {
    Ok(match force_type {
        S8 => value.parse::<i8>()?.into(),
        S16 => value.parse::<i16>()?.into(),
        S32 => value.parse::<i32>()?.into(),
        S64 => value.parse::<i64>()?.into(),
        S128 => value.parse::<i128>()?.into(),
        U8 => value.parse::<u8>()?.into(),
        U16 => value.parse::<u16>()?.into(),
        U32 => value.parse::<u32>()?.into(),
        U64 => value.parse::<u64>()?.into(),
        U128 => value.parse::<u128>()?.into(),
        F32 => value.parse::<f32>()?.into(),
        F64 => value.parse::<f64>()?.into(),
    })
}

fn get_forced_array(parse_data: &mut ParseData, force_type: ForceCategory) -> StdResult {
    parse_data.next_char();
    parse_data.next_non_whitespace_peek();
    match force_type {
        S8 => arrays::s8(parse_data),        
        S16 => arrays::s16(parse_data),        
        S32 => arrays::s32(parse_data),        
        S64 => arrays::s64(parse_data),        
        S128 => arrays::s128(parse_data),        
        U8 => arrays::u8(parse_data),        
        U16 => arrays::u16(parse_data),        
        U32 => arrays::u32(parse_data),        
        U64 => arrays::u64(parse_data),        
        U128 => arrays::u128(parse_data),        
        F32 => arrays::f32(parse_data),
        F64 => arrays::f64(parse_data),
    }
}

pub(super) fn array(parse_data: &mut ParseData) -> WIPResult {
    parse_data.next_char();
    match parse_data.next_non_whitespace_peek() {
        Some('|' | '"') => arrays::strings(parse_data),
        Some('{') => arrays::structs(parse_data),
        Some('t' | 'f') => Ok(arrays::bool(parse_data)?.into()),
        Some(c) => Err(illegal_char_err(c, parse_data)),
        None => Err(Error::UnexpectedEOF)
    }
}

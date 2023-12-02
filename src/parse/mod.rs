//! Functions for parsing stuff to [`PmlStructs`](crate::PmlStruct).
use std::{fs, rc::Rc, cell::RefCell};
use crate::{PmlStruct, errors::ParseError as Error};

mod types;
mod get_value;
pub(crate) use types::{ISElem, KeyType, ParseData, WIPElement, WIPStruct, MetaInfo};
use types::TerminatorType;

/// Parses a file to a [`PmlStruct`](crate::PmlStruct).
///
/// Takes the path to a file, parses it, and returns a `PmlStruct` if the file could be parsed
/// successfully, or an error if one occured.
///
/// # Errors
/// This function returns a [`ParseError`](crate::errors::ParseError) if the file could not be
/// opened, or if it contains invalid syntax or data.
pub fn file(file: &str) -> Result<PmlStruct, Error> {
    let file_content = fs::read_to_string(file)?;
    parse_pml_string(&file_content)
}

fn parse_pml_string(input: &str) -> Result<PmlStruct, Error> {
    let mut parse_data = ParseData::init(input);
    let temp_struct = Rc::new(RefCell::new(WIPStruct::init()));
    parse_data.add_nested_ref(temp_struct.clone());
    
    get_meta_info(&mut parse_data)?;
    while parse_data.has_next_non_whitespace() {
        let (key, value) = get_key_value_pair(&mut parse_data)?;
        temp_struct.borrow_mut().add(key, value)?;
        parse_data.try_skip_comment();
    }
    loop {
        let (no_change, done) = temp_struct.borrow_mut().resolve_inc_strings();
        let (no_change2, done2) = temp_struct.borrow().resolve_inc_strings_recursive();
        if done && done2 {
            break;
        }
        if no_change && no_change2{
            return Err(Error::IllegalDependency)
        }
    }
    let struct_arrays = temp_struct.borrow().resolve_struct_arrays()?;
    for (k, v) in  struct_arrays {
        temp_struct.borrow_mut().finished_elements.insert(k, v);
    }
    let final_struct = temp_struct.borrow_mut().resolve_inc_structs();
    final_struct
}

fn illegal_char_err(c: char, pd: &ParseData) -> Error {
    Error::IllegalCharacter {
        char: c,
        line: pd.line,
        col: pd.column
    }
}

fn is_char_reserved(c: char) -> bool {
    ['=', ';', ',', '<', '>', '{', '}', '(', ')', '"', '[', ']', ':', '|', '.', '+', '$', '!', '?', '#'].into_iter().any(|r| r == c)
}

fn get_key_value_pair(parse_data: &mut ParseData) -> Result<(String, WIPElement), Error> {
    let key = match parse_data.next_non_whitespace_peek() {
        Some('"') => get_quoted_key(parse_data),
        Some(c) if is_char_reserved(c) => Err(illegal_char_err(c, parse_data)),
        Some(_) => get_unquoted_key(parse_data),
        None => unreachable!(),
    }?;
    parse_data.try_skip_comment();
    let value = match parse_data.next_non_whitespace_peek() {
        Some('|'|'"') => {
            parse_data.add_nested_name(key.clone());
            let res = get_value::string(parse_data, TerminatorType::Struct)?.into();
            parse_data.drop_last_nested_name();
            res
        }
        Some('t' | 'f') => get_value::bool(parse_data, TerminatorType::Struct)?.into(),
        Some('<') => get_value::forced(parse_data, TerminatorType::Struct, &key)?,
        Some('{') => {
            parse_data.add_nested_name(key.clone());
            let res = get_value::pml_struct(parse_data, TerminatorType::Struct)?.into();
            parse_data.drop_last_nested_name();
            res
        }
        Some('.' | '-') => get_value::number(parse_data, TerminatorType::Struct)?.into(),
        Some(c) if c.is_ascii_digit() => get_value::number(parse_data, TerminatorType::Struct)?.into(),
        Some(c) => Err(illegal_char_err(c, parse_data))?,
        None => Err(Error::UnexpectedEOF)?,
    };
    Ok((key, value))
}

fn get_quoted_key(parse_data: &mut ParseData) -> Result<String, Error> {
    parse_data.next_char();
    let mut key = String::new();
    while let Some(c) = parse_data.next_char() {
        match c {
            '"' => {
                if key.is_empty() {
                    return Err(Error::InvalidKey)
                }
                return match parse_data.next_non_whitespace() {
                    Some('=') => Ok(key),
                    Some('#') => {
                        parse_data.skip_comment();
                        match parse_data.next_non_whitespace() {
                            Some('=') => Ok(key),
                            Some(c) => Err(illegal_char_err(c, parse_data)),
                            None => Err(Error::UnexpectedEOF)
                        }
                    }
                    Some(c) => Err(illegal_char_err(c, parse_data)),
                    None => Err(Error::UnexpectedEOF)
                }
            }
            c if is_char_reserved(c) => return Err(Error::InvalidKey),
            c => key.push(c)
        }
    }
    Err(Error::UnexpectedEOF)
}

fn get_unquoted_key(parse_data: &mut ParseData) -> Result<String, Error> {
    let mut key = String::new();
    while let Some(c) = parse_data.next_char() {
        match c {
            '=' => {
                if key.is_empty() {
                    return Err(Error::InvalidKey)
                }
                return Ok(key)
            }
            '#' => {
                parse_data.skip_comment();
                match parse_data.next_non_whitespace() {
                    Some('=') => return Ok(key),
                    Some(c) => return Err(illegal_char_err(c, parse_data)),
                    None => return Err(Error::UnexpectedEOF)
                }
            }
            c if c.is_whitespace() => {
                return match parse_data.next_non_whitespace() {
                    Some('=') => Ok(key),
                    Some(c) => Err(illegal_char_err(c, parse_data)),
                    None => Err(Error::UnexpectedEOF)
                }
            }
            c if is_char_reserved(c) => return Err(Error::InvalidKey),
            c => key.push(c)
        }
    }
    Err(Error::UnexpectedEOF)
}

fn get_meta_info(parse_data: &mut ParseData) -> Result<MetaInfo, Error> {
    let mut meta_info = MetaInfo::init();
    if parse_data.next_non_whitespace_peek() == Some('#') {
        let ident = get_meta_ident(parse_data)?;
        if ident == "version" {
            meta_info.parse_version(parse_data)?;
        }
        else {
            handle_meta_info(parse_data, &ident, &mut meta_info)?;
        }
    }
    while let Some('#') = parse_data.next_non_whitespace_peek() {
        let ident = get_meta_ident(parse_data)?;
        handle_meta_info(parse_data, &ident, &mut meta_info)?;
    }
    Ok(meta_info)
}

fn get_meta_ident(parse_data: &mut ParseData) -> Result<String, Error> {
    parse_data.next_char();
    let mut ident = String::new();
    loop {
        match parse_data.next_char() {
            Some(c) if c.is_whitespace() => break,
            Some(c) => ident.push(c),
            None => return Err(Error::UnexpectedEOF)
        }
    }
    Ok(ident)
}

fn handle_meta_info(parse_data: &mut ParseData, ident: &str, meta_info: &mut MetaInfo) -> Result<(), Error> {
    match ident {
        "def" => meta_info.add_struct_template(parse_data),
        _ => {
            parse_data.skip_comment();
            Ok(())
        }
    }
}

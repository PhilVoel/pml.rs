use crate::{ParseError as Error, parse::{ParseData, TerminatorType}};
use super::{get_number_type_and_string, WIPResult};

pub(super) fn strings(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    let mut count = 0;
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        array.push((count, super::string(parse_data, TerminatorType::Array)?));
        count += 1;
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn structs(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    let mut count = 0;
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        array.push((count, super::pml_struct(parse_data, TerminatorType::Array)?));
        parse_data.drop_last_nested_ref();
        count += 1;
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn bool(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        array.push(super::bool(parse_data, TerminatorType::Array)?);
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn f32(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<f32>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn f64(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<f64>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn i8(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<i8>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn i16(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<i16>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn i32(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        parse_data.next_non_whitespace_peek();
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<i32>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn i64(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<i64>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn i128(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<i128>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn u8(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<u8>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn u16(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<u16>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn u32(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<u32>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn u64(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<u64>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

pub(super) fn u128(parse_data: &mut ParseData) -> WIPResult {
    let mut array = Vec::new();
    while parse_data.last_char != ']' {
        match parse_data.next_non_whitespace_peek() {
            None => return Err(Error::UnexpectedEOF),
            Some(']') => {
                parse_data.next_char();
                break;
            }
            Some(_) => ()
        }
        let string = get_number_type_and_string(parse_data, TerminatorType::Array)?.1;
        match string.parse::<u128>() {
            Ok(num) => array.push(num),
            Err(e) => return Err(Error::ParseNumberError {
                line: parse_data.line,
                value: string,
                error: e.into()
            })
        }
        parse_data.try_skip_comment();
    }
    Ok(array.into())
}

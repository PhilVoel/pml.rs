use std::{io::Error as IoError, num::{ParseFloatError, ParseIntError}};

#[derive(Debug)]
pub enum ParseError {
    AlreadyExists {
        key: String,
    },
    FileAccess(IoError),
    InvalidKey,
    IllegalCharacter{
        char: char,
        line: u32,
        col: u32
    },
    IllegalDependency,
    NotAnExistingStruct(String),
    ParseNumberError{
        line: u32,
        value: String,
        error: ParseNumberError
    },
    UnexpectedEOF,
    UnknownForcedType{
        key: String,
        type_name: String
    }
}

#[derive(Debug)]
pub enum ParseNumberError {
    Int(ParseIntError),
    Float(ParseFloatError)
}

#[derive(Debug)]
pub enum GetError {
    DoesNotExits,
    InvalidType
}

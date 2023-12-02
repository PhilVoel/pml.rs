use crate::errors::{ParseError, ParseNumberError};
use std::num::{ParseIntError, ParseFloatError};

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        ParseError::FileAccess(e)
    }
}

impl From<ParseIntError> for ParseNumberError {
    fn from(e: ParseIntError) -> Self {
        ParseNumberError::Int(e)
    }
}
impl From<ParseFloatError> for ParseNumberError {
    fn from(e: ParseFloatError) -> Self {
        ParseNumberError::Float(e)
    }
}

use crate::{Error, ParseNumberError};
use std::num::{ParseIntError, ParseFloatError};

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::FileAccess(e)
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

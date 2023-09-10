use crate::parse::{Error, NumberError};
use std::num::{ParseIntError, ParseFloatError};

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::FileAccess(e)
    }
}

impl From<ParseIntError> for NumberError {
    fn from(e: ParseIntError) -> Self {
        NumberError::Int(e)
    }
}
impl From<ParseFloatError> for NumberError {
    fn from(e: ParseFloatError) -> Self {
        NumberError::Float(e)
    }
}

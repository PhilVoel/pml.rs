use crate::Error;
use std::num::{ParseIntError, ParseFloatError};

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::FileAccess(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}
impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Error::ParseFloatError(e)
    }
}

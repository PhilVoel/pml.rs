//! Module containing any errors that may be returned when parsing, inserting or getting data.

use std::{io::Error as IoError, num::{ParseFloatError, ParseIntError}};

/// Errors that may occur when [parsing](crate::parse) a PML file or [adding](crate::PmlStruct::add) data to a [`PmlStruct`](crate::PmlStruct).
#[derive(Debug)]
pub enum ParseError {
    /// The provided key was already used for another value.
    AlreadyExists {
        /// The key that was already used.
        key: String,
    },
    /// The file with the provided path could not be opened.
    FileAccess(
        /// The error thrown by [fs](std::fs).
        IoError
    ),
    /// The key is not valid.
    InvalidKey,
    /// The character is not allowed at that position.
    IllegalCharacter{
        /// The character that is not allowed.
        char: char,
        /// The line where the character appears.
        line: u32,
        /// The column where the character appears.
        col: u32
    },
    /// An illegal dependecy was found in a combined string. This may be because the dependency
    /// does not exist, or because strings depend on each other in a circular way.
    IllegalDependency,
    /// The element could not be inserted because the key implies a struct where a non-struct
    /// element was found.
    NotAStruct(String),
    /// An error occured while parsing a number.
    ParseNumberError{
        /// The line in which the number appears.
        line: u32,
        /// The literal value that was provided.
        value: String,
        /// The error that occured while parsing the number.
        error: ParseNumberError
    },
    /// The end of the file was reached unexpectedly.
    UnexpectedEOF,
    /// The provided forced type does not exist.
    UnknownForcedType{
        /// The key of the element that was supposed to be forced.
        key: String,
        /// The typename that was provided.
        type_name: String
    },
}

/// Errors that may occur when parsing a number. This can occur because the provided number could
/// not pe parsed at all (usually when the number is too big to fit into Rust's primitive number
/// types), or because the number was parsed, but could not be converted to the provided forced type.
#[derive(Debug)]
pub enum ParseNumberError {
    /// The number could not be parsed as a signed or unsigned integer.
    Int(ParseIntError),
    /// The number could not be parsed as a floating point number.
    Float(ParseFloatError)
}

/// Errors that may occur when [getting data](crate::PmlStruct::get) from a [`PmlStruct`](crate::PmlStruct).
#[derive(Debug)]
pub enum GetError {
    /// The requested element does not exist.
    DoesNotExits,
    /// The element could not be returned as the requested type.
    InvalidType
}

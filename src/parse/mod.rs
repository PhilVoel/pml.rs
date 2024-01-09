//! Functions for parsing stuff to [`PmlStructs`](crate::PmlStruct).
use std::fs;
use crate::{PmlStruct, errors::ParseError as Error};

mod tree;
use tree::ParseTree;

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
    let mut tree = ParseTree::try_from(input)?.parse_meta_info()?;
    todo!()
}

use std::{iter::Peekable, str::Chars};
use crate::{meta_info::{Version, Template}, ParseError};

impl Version {
    pub fn parse(chars: &mut Peekable<Chars<'_>>, line: &mut u32, col: &mut u32) -> Result<Self, ParseError> {
        enum ParseStatus {
            Before,
            Major,
            Minor,
            Done,
        }
        use ParseStatus::*;

        let mut status = ParseStatus::Before;
        let mut major = String::new();
        let mut minor = String::new();
        while let Some(char) = chars.next() {
            if char == '\n' {
                *line += 1;
                *col = 1;
            } else {
                *col += 1;
            }
            match status {
                Before if char.is_whitespace() => (),
                Before if char.is_ascii_digit() => {
                    major.push(char);
                    status = Major;
                }
                Before => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),
                
                Major if char.is_ascii_digit() => major.push(char),
                Major if char == '.' => status = Minor,
                Major => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),

                Minor if char.is_ascii_digit() => minor.push(char),
                Minor if char.is_whitespace() => status = Done,
                Done | Minor if char == ';' => {
                    let major = major.parse().map_err(|_| ParseError::InvalidVersion)?;
                    let minor = minor.parse().map_err(|_| ParseError::InvalidVersion)?;
                    return Ok(Self{major, minor});
                }
                Minor => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),

                Done if !char.is_whitespace() => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),
                Done => (),
            }
        }
        Err(ParseError::UnexpectedEOF)
    }
}

impl Template {
    pub fn parse(value: &mut Peekable<Chars<'_>>, line: &mut u32, col: &mut u32) -> Result<Self, ParseError> {
        todo!()
    }
}

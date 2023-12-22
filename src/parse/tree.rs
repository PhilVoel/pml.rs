use crate::ParseError;
use std::{str::Chars, collections::HashMap, rc::{Rc, Weak}};

pub struct ParseTree {
    root: Rc<Node>,
    meta_info: Vec<String>,
}

struct Node {
    value: Content,
    parent: Option<Weak<Node>>,
}

enum Content {
    Value(String),
    Children(HashMap<String, Rc<Node>>),
}

enum ParseStatus {
    Start,
    MetaInfo,
    QuotedKey,
    UnquotedKey,
    AfterKey,
    AfterEquals,
    Value,
}

struct ParseState<'a> {
    chars: Chars<'a>,
    line: u32,
    col: u32,
}

use ParseStatus::{Start, MetaInfo, QuotedKey, UnquotedKey, AfterKey, AfterEquals, Value};

impl TryFrom<&str> for ParseTree {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        let mut line = 1;
        let mut col = 0;


        let mut parse_state = ParseState {
            chars,
            line,
            col,
        };
        let root = get_struct(&mut parse_state)?;
        if let Some(char) = parse_state.chars.next() {
            return Err(ParseError::IllegalCharacter{char, line: parse_state.line, col: parse_state.col});
        }
    }
}

fn get_struct(parse_state: &mut ParseState) -> Result<Node, ParseError> {
    let mut status = Start;
    let mut current = String::new();
    let mut meta_infos = Vec::new();
    let mut key = String::new();
    while let Some(char) = parse_state.chars.next() {
        if char == '\n' {
            parse_state.line += 1;
            parse_state.col = 1;
        } else {
            parse_state.col += 1;
        }
        let line = parse_state.line;
        let col = parse_state.col;
        match status {
            Start if char.is_whitespace() => continue,
            Start if char == '#' => status = MetaInfo,
            Start if char == '"' => status = QuotedKey,
            Start if is_char_reserved(char) => return Err(ParseError::IllegalCharacter{char, line, col}),
            Start => {
                current.push(char);
                status = UnquotedKey;
            }

            MetaInfo if char == ';' => {
                meta_infos.push(current);
                current = String::new();
                status = Start;
            }
            MetaInfo => current.push(char),

            QuotedKey if char == '"' => {
                key = current;
                current = String::new();
                status = AfterKey;
            }
            QuotedKey if char == ' ' => current.push(char),
            QuotedKey if is_char_reserved(char) => return Err(ParseError::IllegalCharacter{char, line, col}),
            QuotedKey => current.push(char),

            UnquotedKey if char.is_whitespace() => {
                key = current;
                current = String::new();
                status = AfterKey;
            }
            UnquotedKey if char == '=' => {
                key = current;
                current = String::new();
                status = AfterEquals;
            }
            UnquotedKey if is_char_reserved(char) => return Err(ParseError::IllegalCharacter{char, line, col}),
            UnquotedKey => current.push(char),

            AfterKey if char.is_whitespace() => continue,
            AfterKey if char == '=' => status = AfterEquals,
            AfterKey => return Err(ParseError::IllegalCharacter{char, line, col}),

            AfterEquals if char.is_whitespace() => continue,
            AfterEquals if char == ';' => return Err(ParseError::IllegalCharacter{char, line, col}),
            AfterEquals if char == '{' => {
                todo!()
            }
            AfterEquals => {
                current.push(char);
                status = Value;
            }

            Value => todo!(),
        }
    }
    Err(ParseError::UnexpectedEOF)
}

fn is_char_reserved(char: char) -> bool {
    ['=', ';', ',', '<', '>', '{', '}', '(', ')', '"', '[', ']', ':', '|', '.', '+', '$', '!', '?', '#'].contains(&char)
}

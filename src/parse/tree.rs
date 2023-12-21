use crate::ParseError;
use std::{collections::HashMap, rc::{Rc, Weak}};

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

use ParseStatus::{Start, MetaInfo, QuotedKey, UnquotedKey, AfterKey, AfterEquals, Value};

impl TryFrom<&str> for ParseTree {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut status = Start;
        let mut current = String::new();
        let mut meta_infos = Vec::new();
        let mut key = String::new();
        let mut line = 1;
        let mut col = 1;
        for char in value.chars() {
            if char == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
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
                AfterEquals => {
                    current.push(char);
                    status = Value;
                }

                Value => todo!(),
            }
        }
        Err(ParseError::UnexpectedEOF)
    }
}

fn is_char_reserved(char: char) -> bool {
    ['=', ';', ',', '<', '>', '{', '}', '(', ')', '"', '[', ']', ':', '|', '.', '+', '$', '!', '?', '#'].contains(&char)
}

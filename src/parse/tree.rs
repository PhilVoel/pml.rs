use crate::{ParseError, meta_info::MetaInfo};
use std::{str::Chars, collections::HashMap, rc::{Rc, Weak}, iter::Peekable, cell::RefCell};

#[derive(Debug)]
pub struct ParseTree {
    root: Rc<RefCell<Node>>,
    meta_info: Vec<MetaInfoState>,
}

#[derive(Debug)]
enum MetaInfoState {
    Raw(String),
    Parsed(MetaInfo),
}

impl From<String> for MetaInfoState {
    fn from(value: String) -> Self {
        MetaInfoState::Raw(value)
    }
}

#[derive(Debug)]
struct Node {
    value: Content,
    parent: Option<Weak<RefCell<Node>>>,
}

#[derive(Debug)]
enum Content {
    Value(ContentValue),
    Children(HashMap<String, Rc<RefCell<Node>>>),
}

#[derive(Debug, Default)]
struct ContentValue {
    value: String,
    line: u32,
    col: u32,
}

#[derive(PartialEq)]
enum ParseStatus {
    Start,
    ReturnAfterSemicolon,
    QuotedKey,
    UnquotedKey,
    AfterKey,
    AfterEquals,
    Value,
}

struct ParseState<'a> {
    chars: Peekable<Chars<'a>>,
    line: u32,
    col: u32,
}

use ParseStatus::{Start, ReturnAfterSemicolon, QuotedKey, UnquotedKey, AfterKey, AfterEquals, Value};

impl TryFrom<&str> for ParseTree {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars().peekable();
        let mut line = 1;
        let mut col = 0;
        let mut inside_meta = false;
        let mut current = String::new();
        let mut meta_info = Vec::new();

        while let Some(char) = chars.peek() {
            if *char == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            if inside_meta && *char == ';' {
                inside_meta = false;
                meta_info.push(current.into());
                current = String::new();
            }
            else if inside_meta {
                current.push(*char);
            }
            else if *char == '#' {
                inside_meta = true;
            }
            else if !char.is_whitespace() {
                col -= 1;
                break;
            }
            chars.next();
        }
        if inside_meta {
            return Err(ParseError::UnexpectedEOF);
        }
        let mut parse_state = ParseState {
            chars,
            line,
            col,
        };
        let root = get_struct(&mut parse_state, None)?;
        if let Some(char) = parse_state.chars.next() {
            return Err(ParseError::IllegalCharacter{char, line: parse_state.line, col: parse_state.col});
        }
        Ok(ParseTree {
            root,
            meta_info,
        })
    }
}

impl ParseTree {
    pub fn parse_meta_info(&mut self) -> Result<&mut Self, ParseError> {
        self.meta_info = self.meta_info.iter()
            .map(|s| match s {
                MetaInfoState::Raw(s) => MetaInfo::try_from(s.as_str()),
                MetaInfoState::Parsed(_) => unreachable!(),
            })
            .collect::<Result<Vec<MetaInfo>, ParseError>>()?
            .into_iter()
            .map(|s| MetaInfoState::Parsed(s))
            .collect();
        Ok(self)
    }

    pub fn parse_values(&mut self) -> Result<&mut Self, ParseError> {
        todo!();
        Ok(self)
    }
}

fn get_struct(parse_state: &mut ParseState, parent: Option<Weak<RefCell<Node>>>) -> Result<Rc<RefCell<Node>>, ParseError> {
    let struct_node = Rc::new(RefCell::new(Node {
        value: Content::Children(HashMap::new()),
        parent
    }));
    let mut status = Start;
    let mut node = struct_node.borrow_mut();
    let elements = match &mut node.value {
        Content::Children(elements) => elements,
        _ => unreachable!(),
    };
    let mut current = String::new();
    let mut key = String::new();
    let mut inside_string = false;
    let mut escape_char = false;
    let mut line_start = 0;
    let mut col_start = 0;
    while let Some(char) = parse_state.chars.next() {
        if char == '\n' {
            parse_state.line += 1;
            parse_state.col = 0;
        } else {
            parse_state.col += 1;
        }
        let line = parse_state.line;
        let col = parse_state.col;
        match status {
            Start if char.is_whitespace() => continue,
            Start if char == '"' => status = QuotedKey,
            Start if char == '}' => status = ReturnAfterSemicolon,
            Start if is_char_reserved(char) => return Err(ParseError::IllegalCharacter{char, line, col}),
            Start => {
                current.push(char);
                status = UnquotedKey;
            }

            ReturnAfterSemicolon if char.is_whitespace() => continue,
            ReturnAfterSemicolon if char == ';' => {
                drop(node);
                return Ok(struct_node)
            }
            ReturnAfterSemicolon => return Err(ParseError::IllegalCharacter{char, line, col}),

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
                let node = get_struct(parse_state, Some(Rc::downgrade(&struct_node)))?;
                elements.insert(key.clone(), node);
                status = Start;
            }
            AfterEquals => {
                line_start = line;
                col_start = col;
                current.push(char);
                status = Value;
                if char == '"' {
                    inside_string = true;
                }
            }

            Value if char == ';' && !inside_string => {
                elements.insert(key.clone(), Rc::new(RefCell::new(Node {
                    value: Content::Value(ContentValue {
                        value: current,
                        line: line_start,
                        col: col_start,
                    }),
                    parent: Some(Rc::downgrade(&struct_node)),
                })));
                status = Start;
                current = String::new();
            }
            Value => {
                current.push(char);
                if inside_string && escape_char {
                    escape_char = false;
                }
                else if inside_string && char == '\\'  {
                    escape_char = true;
                }
                else if char == '"' {
                    inside_string = !inside_string;
                }
            }
        }
    }
    if status == Start {
        drop(node);
        Ok(struct_node)
    }
    else {
        Err(ParseError::UnexpectedEOF)
    }
}

fn is_char_reserved(char: char) -> bool {
    ['=', ';', ',', '<', '>', '{', '}', '(', ')', '"', '[', ']', ':', '|', '.', '+', '$', '!', '?', '#'].contains(&char)
}

use crate::{ParseError, meta_info::{MetaInfo, Version, Template}};
use std::{str::Chars, collections::HashMap, rc::{Rc, Weak}, iter::Peekable, cell::RefCell};
use super::is_char_reserved;

#[derive(Debug)]
pub struct ParseTree {
    root: Rc<RefCell<Node>>,
    meta_info: Vec<MetaInfo>,
}

#[derive(Debug)]
struct Node {
    value: Content,
    parent: Option<Weak<RefCell<Node>>>,
}

#[derive(Debug)]
enum Content {
    Value(ContentValue),
    Struct {
        template: Option<String>,
        children: HashMap<String, Rc<RefCell<Node>>>
    },
}

#[derive(Debug, Default)]
struct ContentValue {
    value: String,
    line: u32,
    col: u32,
}

impl TryFrom<&str> for ParseTree {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        #[derive(PartialEq)]
        enum ParseStatus {
            Start,
            Descriptor,
            SkipToNextLine
        }
        use ParseStatus::*;

        let mut chars = value.chars().peekable();
        let mut line = 1;
        let mut col = 0;
        let mut current = String::new();
        let mut meta_info = Vec::new();
        let mut status = Start;

        while let Some(char) = chars.peek() {
            if *char == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            match status {
                Start if char.is_whitespace() => (),
                Start if *char == '#' => status = Descriptor,
                Start => {
                    col -= 1;
                    break;
                }

                Descriptor if char.is_whitespace() => {
                    match current.as_str() {
                        "version" => meta_info.push(Version::parse(&mut chars, &mut line, &mut col)?.into()),
                        "def" => meta_info.push(Template::parse(&mut chars, &mut line, &mut col)?.into()),
                        _ => (),
                    }
                    status = SkipToNextLine;
                },
                Descriptor if is_char_reserved(*char) => return Err(ParseError::IllegalCharacter{char: *char, line, col}),
                Descriptor => current.push(*char),

                SkipToNextLine if *char == '\n' => status = Start,
                SkipToNextLine => (),
            }
            chars.next();
        }
        if status != Start && status != SkipToNextLine {
            return Err(ParseError::UnexpectedEOF);
        }
        let root = get_struct(&mut chars, None, false, &mut line, &mut col)?;
        if let Some(char) = chars.next() {
            return Err(ParseError::IllegalCharacter{char, line, col});
        }
        Ok(ParseTree {
            root,
            meta_info,
        })
    }
}

fn get_struct(chars: &mut Peekable<Chars>, parent: Option<Weak<RefCell<Node>>>, get_template: bool, line: &mut u32, col: &mut u32) -> Result<Rc<RefCell<Node>>, ParseError> {
    #[derive(PartialEq)]
    enum ParseStatus {
        Start,
        ReturnAfterSemicolon,
        QuotedKey,
        UnquotedKey,
        AfterKey,
        AfterEquals,
        Value,
        Comment(Box<ParseStatus>),
    }
    use ParseStatus::*;

    let struct_node = Rc::new(RefCell::new(Node {
        parent,
        value: Content::Struct{
            children: HashMap::new(),
            template: None,
        },
    }));
    let mut status = Start;
    let mut current = String::new();
    let mut key = String::new();
    let mut inside_string = false;
    let mut escape_char = false;
    let mut line_start = 0;
    let mut col_start = 0;
    if get_template {
        let mut done = false;
        let mut wait_for_curly_bracket = false;
        while let Some(char) = chars.next() {
            if char == '\n' {
                *line += 1;
                *col = 0;
            } else {
                *col += 1;
            }
            if done && char == ';' {
                struct_node.borrow_mut().value = Content::Struct{
                    children: HashMap::new(),
                    template: Some(current),
                };
                return Ok(struct_node);
            }
            else if done && char == '+' {
                struct_node.borrow_mut().value = Content::Struct{
                    children: HashMap::new(),
                    template: Some(current),
                };
                current = String::new();
                wait_for_curly_bracket = true;
            }
            else if wait_for_curly_bracket && char == '{' {
                break;
            }
            else if (done || wait_for_curly_bracket) && !char.is_whitespace() {
                return Err(ParseError::IllegalCharacter{char, line: *line, col: *col});
            }
            else if char == ')' {
                done = true;
            }
            else if !done && !wait_for_curly_bracket {
                current.push(char);
            }
        }
    }
    let mut node = struct_node.borrow_mut();
    let elements = match &mut node.value {
        Content::Struct{children: elements, ..} => elements,
        _ => unreachable!(),
    };
    while let Some(char) = chars.next() {
        if char == '\n' {
            *line += 1;
            *col = 0;
        } else {
            *col += 1;
        }
        match status {
            Start if char.is_whitespace() => continue,
            Start if char == '"' => status = QuotedKey,
            Start if char == '}' => status = ReturnAfterSemicolon,
            Start if char == '#' => status = Comment(Box::new(Start)),
            Start if is_char_reserved(char) => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),
            Start => {
                current.push(char);
                status = UnquotedKey;
            }

            ReturnAfterSemicolon if char.is_whitespace() => continue,
            ReturnAfterSemicolon if char == '#' => status = Comment(Box::new(ReturnAfterSemicolon)),
            ReturnAfterSemicolon if char == ';' => {
                drop(node);
                return Ok(struct_node)
            }
            ReturnAfterSemicolon => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),

            QuotedKey if char == '"' => {
                key = current;
                current = String::new();
                status = AfterKey;
            }
            QuotedKey if char == ' ' => current.push(char),
            QuotedKey if is_char_reserved(char) => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),
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
            UnquotedKey if char == '#' => status = Comment(Box::new(AfterKey)),
            UnquotedKey if is_char_reserved(char) => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),
            UnquotedKey => current.push(char),

            AfterKey if char.is_whitespace() => continue,
            AfterKey if char == '=' => status = AfterEquals,
            AfterKey if char == '#' => status = Comment(Box::new(AfterKey)),
            AfterKey => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),

            AfterEquals if char.is_whitespace() => continue,
            AfterEquals if char == '#' => status = Comment(Box::new(AfterEquals)),
            AfterEquals if char == ';' => return Err(ParseError::IllegalCharacter{char, line: *line, col: *col}),
            AfterEquals if char == '{' || char == '(' => {
                let node = get_struct(chars, Some(Rc::downgrade(&struct_node)), char == '(', line, col)?;
                elements.insert(key.clone(), node);
                status = Start;
            }
            AfterEquals => {
                line_start = *line;
                col_start = *col;
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
            Value if char == '#' && !inside_string => status = Comment(Box::new(Value)),
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
            Comment(prev) if char == '\n' => status = *prev,
            Comment(_) => (),
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

impl ParseTree {
    pub fn try_parse_strings(&mut self) -> Result<&mut Self, ParseError> {
        todo!()
    }
}

use std::{collections::HashMap, io::Error as IoError, num::{ParseFloatError, ParseIntError}};

mod impls;
pub mod parse;

mod elem {
    use crate::parse::TextState;
    #[derive(Debug, Clone)]
    pub enum Element {
        PmlStruct(Box<crate::PmlStruct>),
        IncompleteString(Vec<(String, TextState)>),
        PmlString(String),
        PmlBool(bool),
        PmlI128(i128),
        PmlI64(i64),
        PmlI32(i32),
        PmlI16(i16),
        PmlI8(i8),
        PmlU128(u128),
        PmlU64(u64),
        PmlU32(u32),
        PmlU16(u16),
        PmlU8(u8),
        PmlF64(f64),
        PmlF32(f32)
    }
}
use elem::Element;

#[derive(Clone, Debug)]
pub struct PmlStruct {
    elements: HashMap<String, Element>,
    own_val: Option<Element>
}

#[derive(Debug)]
pub enum ParseNumberError {
    Int(ParseIntError),
    Float(ParseFloatError)
}

#[derive(Debug)]
pub enum Error {
    AlreadyExists {
        key: String,
        old_val: Element,
        line: u32
    },
    CircularDependency(Vec<String>),
    EmptyStruct{
        key: String,
        closing_line: u32,
        closing_col: u32
    },
    FileAccess(IoError),
    IllegalCharacter{
        char: char,
        line: u32,
        col: u32
    },
    NotAnExistingStruct(String),
    ParseNumberError{
        line: u32,
        value: String,
        error: ParseNumberError
    },
    UnexpectedEOF,
    UnfulfilledDependency{
        key: String,
        dependency: String
    },
    UnknownForcedType{
        key: String,
        type_name: String
    }
}

impl<'a> PmlStruct {
    pub fn get<T>(&'a self, key: &str) -> Option<T>
        where
        T: From<&'a Element>
        {
            if key.is_empty() {
                return Some(T::from(self.own_val.as_ref()?));
            }
            match key.split_once('.') {
                None => self.elements.get(key).map(|elem| T::from(elem)),
                Some((first, rest)) => match self.elements.get(first)? {
                    Element::PmlStruct(s) => s.get::<T>(rest),
                    _ => None
                }
            }
        }

    pub fn add<T>(&mut self, key: String, elem: T) -> Result<(), Error>
        where
        T: Into<Element>
        {
            match key.split_once('.') {
                None => {
                    match self.elements.insert(key.clone(), elem.into()) {
                        Some(old_val) => Err(Error::AlreadyExists{key, old_val, line: 0}),
                        None => Ok(())
                    }
                }
                Some((first, rest)) => match self.elements.get_mut(first) {
                    Some(Element::PmlStruct(s)) => s.add(String::from(rest), elem),
                    _ => Err(Error::NotAnExistingStruct(String::from(first)))
                }
            }
        }
}

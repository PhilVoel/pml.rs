use std::{collections::HashMap, io::Error as IoError, num::{ParseFloatError, ParseIntError}};

mod impls;
pub mod parse;

mod elem {
    use crate::PmlStruct;

    #[derive(Debug, Clone)]
    pub enum Element {
        PmlArray(ArrayElement),
        PmlBool(bool),
        PmlString(String),
        PmlStruct(Box<PmlStruct>),
        PmlF32(f32),
        PmlF64(f64),
        PmlI8(i8),
        PmlI16(i16),
        PmlI32(i32),
        PmlI64(i64),
        PmlI128(i128),
        PmlU8(u8),
        PmlU16(u16),
        PmlU32(u32),
        PmlU64(u64),
        PmlU128(u128),
    }

    #[derive(Debug, Clone)]
    pub enum ArrayElement {
        PmlBool(Vec<bool>),
        PmlStruct(Vec<PmlStruct>),
        PmlString(Vec<String>),
        PmlF32(Vec<f32>),
        PmlF64(Vec<f64>),
        PmlI8(Vec<i8>),
        PmlI16(Vec<i16>),
        PmlI32(Vec<i32>),
        PmlI64(Vec<i64>),
        PmlI128(Vec<i128>),
        PmlU8(Vec<u8>),
        PmlU16(Vec<u16>),
        PmlU32(Vec<u32>),
        PmlU64(Vec<u64>),
        PmlU128(Vec<u128>),
    }
}
use elem::Element;

#[derive(Clone, Debug)]
pub struct PmlStruct {
    elements: HashMap<String, Element>,
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
    },
    CircularDependency(Vec<String>),
    FileAccess(IoError),
    InvalidKey,
    IllegalCharacter{
        char: char,
        line: u32,
        col: u32
    },
    IllegalDependency,
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
            if key.starts_with('.') || key.ends_with('.') || key.is_empty() {
                return Err(Error::InvalidKey);
            }
            match key.split_once('.') {
                None => {
                    match self.elements.insert(key.clone(), elem.into()) {
                        Some(_) => Err(Error::AlreadyExists{key}),
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

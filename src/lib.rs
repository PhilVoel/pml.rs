use std::{collections::HashMap, io::Error as IoError, num::{ParseFloatError, ParseIntError}};

mod impls;
pub mod parse;

mod elem {
    use crate::parse::StringState;
    #[derive(Debug, Clone)]
    pub enum Element {
        IncompleteString(Vec<(String, StringState)>),
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

pub struct PmlStruct {
    elements: HashMap<String, Element>
}

#[derive(Debug)]
pub enum Error {
    AlreadyExists {
        key: String,
        old_val: Element,
        new_val: Element
    },
    CircularDependency(Vec<String>),
    FileAccess(IoError),
    UnfulfilledDependency{
        key: String,
        dependency: String
    },
    Parse,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    UnknownTypeForced{
        key: String,
        type_name: String
    }
}

impl<'a> PmlStruct {
    pub fn get<T>(&'a self, key: &str) -> Option<T>
        where
        T: From<&'a Element>
        {
            self.elements.get(key).map(|elem| T::from(elem))
        }

    pub fn add<T>(&mut self, key: String, elem: T) -> Result<(), Error>
        where
        T: Into<Element> + Clone
        {
            match self.elements.insert(key.clone(), elem.clone().into()) {
            Some(old_val) => Err(Error::AlreadyExists{key, old_val, new_val:elem.into()}),
                None => Ok(())
            }
        }
}


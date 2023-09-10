//! A library for parsing and working with the PML language.
//!
//! PML is a language for storing complex data in a human-readable format.
//! For more information, see the [PML specification](https://gist.github.com/PhilVoel/00a6acb11257d00a84451f9df4d5b340).

use std::collections::HashMap;

mod impls;
pub mod parse;
pub mod errors;
pub use errors::{ParseError, ParseNumberError, GetError};

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

/// A container that holds key-value pairs of data.
impl<'a> PmlStruct {
    /// Returns the value of the element with the provided key.
    ///
    /// Takes a key to the element that should be returned. Returns the element as type `<T>` if the
    /// conversion could be performed, or an error if one occured.
    ///
    /// # Errors
    /// This function returns an error if the element does not exist, or if the element exists, but
    /// could not be converted to the requested type.
    pub fn get<T>(&'a self, key: &str) -> Result<T, GetError>
        where
        T: TryFrom<&'a Element, Error = GetError>
        {
            match key.split_once('.') {
                None => match self.elements.get(key).map(|elem| T::try_from(elem)) {
                    None => Err(GetError::DoesNotExits),
                    Some(res) => res
                }
                Some((first, rest)) => match self.elements.get(first) {
                    Some(Element::PmlStruct(s)) => s.get::<T>(rest),
                    Some(_) => Err(GetError::InvalidType),
                    None => Err(GetError::DoesNotExits)
                }
            }
        }

    /// Adds an element to the struct.
    ///
    /// Takes a key and a value that can be saved in a `PmlStruct`. Should the key point into
    /// an unexisting struct, the struct will be created.
    ///
    /// # Errors
    /// This function returns an error if the key is invalid, if the element already exists, or if
    /// the key points into an existing element that is not a struct.
    pub fn add<T>(&mut self, key: String, elem: T) -> Result<(), ParseError>
        where
        T: Into<Element>
        {
            if key.starts_with('.') || key.ends_with('.') || key.is_empty() {
                return Err(ParseError::InvalidKey);
            }
            match key.split_once('.') {
                None => {
                    match self.elements.insert(key.clone(), elem.into()) {
                        Some(_) => Err(ParseError::AlreadyExists{key}),
                        None => Ok(())
                    }
                }
                Some((first, rest)) => match self.elements.get_mut(first) {
                    Some(Element::PmlStruct(s)) => s.add(String::from(rest), elem),
                    Some(_) => Err(ParseError::NotAStruct(String::from(first))),
                    None => {
                        let mut s = PmlStruct{elements: HashMap::new()};
                        s.add(String::from(rest), elem)?;
                        self.elements.insert(String::from(first), Element::PmlStruct(Box::new(s)));
                        Ok(())
                    }
                }
            }
        }
}

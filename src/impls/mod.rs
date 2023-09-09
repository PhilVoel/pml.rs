use crate::{elem::{Element, ArrayElement}, PmlStruct, GetError};
macro_rules! add_primitive {
    ($pml_elem:ident, $type:ty $(,$casts:ident)*) => {
        impl From<$type> for Element {
            fn from(f: $type) -> Self {
                Element::$pml_elem(f)
            }
        }

        impl TryFrom<&Element> for $type {
            type Error = GetError;
            fn try_from(elem: &Element) -> Result<Self, Self::Error> {
                match elem {
                    Element::$pml_elem(e) => Ok(*e),
                    $(
                        Element::$casts(e) => Ok(*e as $type),
                        )*
                        _ => Err(Self::Error::InvalidType)
                }
            }
        }

        impl<'a> TryFrom<&'a Element> for &'a $type {
            type Error = GetError;
            fn try_from(elem: &'a Element) -> Result<Self, Self::Error> {
                match elem {
                    Element::$pml_elem(e) => Ok(e),
                    _ => Err(Self::Error::InvalidType)
                }
            }
        }

        impl From<Vec<$type>> for Element {
            fn from(f: Vec<$type>) -> Self {
                Element::PmlArray(ArrayElement::$pml_elem(f))
            }
        }

        impl TryFrom<&Element> for Vec<$type> {
            type Error = GetError;
            fn try_from(elem: &Element) -> Result<Self, Self::Error> {
                match elem {
                    Element::PmlArray(ArrayElement::$pml_elem(e)) => Ok(e.clone()),
                    $(
                        Element::PmlArray(ArrayElement::$casts(e)) => Ok(e.iter().map(|n| *n as $type).collect()),
                    )*
                    _ => Err(Self::Error::InvalidType)
                }
            }
        }

        impl<'a> TryFrom<&'a Element> for &'a Vec<$type> {
            type Error = GetError;
            fn try_from(elem: &'a Element) -> Result<Self, Self::Error> {
                match elem {
                    Element::PmlArray(ArrayElement::$pml_elem(e)) => Ok(e),
                    _ => Err(Self::Error::InvalidType)
                }
            }
        }
    }
}

macro_rules! generic_non_primitive {
    ($pml_elem:ident, $type:ty) => {
        impl<'a> TryFrom<&'a Element> for &'a $type {
            type Error = GetError;
            fn try_from(elem: &'a Element) -> Result<Self, Self::Error> {
                match elem {
                    Element::$pml_elem(f) => Ok(f),
                    _ => Err(Self::Error::InvalidType)
                }
            }
        }

        impl From<Vec<$type>> for Element {
            fn from(f: Vec<$type>) -> Self {
                Element::PmlArray(ArrayElement::$pml_elem(f))
            }
        }

        impl<'a> TryFrom<&'a Element> for &'a Vec<$type> {
            type Error = GetError;
            fn try_from(elem: &'a Element) -> Result<Self, Self::Error> {
                match elem {
                    Element::PmlArray(ArrayElement::$pml_elem(arr)) => Ok(arr),
                    _ => Err(Self::Error::InvalidType)
                }
            }
        }
    }
}

add_primitive!(PmlBool, bool);

add_primitive!(PmlF32, f32);
add_primitive!(PmlF64, f64, PmlF32);

add_primitive!(PmlU8, u8);
add_primitive!(PmlU16, u16, PmlU8);
add_primitive!(PmlU32, u32, PmlU8, PmlU16);
add_primitive!(PmlU64, u64, PmlU8, PmlU16, PmlU32);
add_primitive!(PmlU128, u128, PmlU8, PmlU16, PmlU32, PmlU64);

add_primitive!(PmlI8, i8);
add_primitive!(PmlI16, i16, PmlI8);
add_primitive!(PmlI32, i32, PmlI8, PmlI16);
add_primitive!(PmlI64, i64, PmlI8, PmlI16, PmlI32);
add_primitive!(PmlI128, i128, PmlI8, PmlI16, PmlI32, PmlI64);

generic_non_primitive!(PmlString, String);
generic_non_primitive!(PmlStruct, PmlStruct);

mod error;
mod pml_elem;
mod string;
mod structs;
mod wip_elem;

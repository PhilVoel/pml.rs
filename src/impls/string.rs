use crate::{Element, elem::ArrayElement as E, GetError};
use std::string::ToString as TS;
use Element::PmlArray as A;

impl From<String> for Element {
    fn from(s: String) -> Self {
        Element::PmlString(s)
    }
}

impl TryFrom<&Element> for String {
    type Error = GetError;
    fn try_from(elem: &Element) -> Result<Self, Self::Error> {
        match elem {
            Element::PmlString(s) => Ok(s.clone()),
            Element::PmlBool(b) => Ok(b.to_string()),
            Element::PmlI8(i) => Ok(i.to_string()),
            Element::PmlI16(i) => Ok(i.to_string()),
            Element::PmlI32(i) => Ok(i.to_string()),
            Element::PmlI64(i) => Ok(i.to_string()),
            Element::PmlI128(i) => Ok(i.to_string()),
            Element::PmlU8(i) => Ok(i.to_string()),
            Element::PmlU16(i) => Ok(i.to_string()),
            Element::PmlU32(i) => Ok(i.to_string()),
            Element::PmlU64(i) => Ok(i.to_string()),
            Element::PmlU128(i) => Ok(i.to_string()),
            Element::PmlF32(f) => Ok(f.to_string()),
            Element::PmlF64(f) => Ok(f.to_string()),
            Element::PmlStruct(_) |
            Element::PmlArray(_) => Err(Self::Error::InvalidType)
        }
    }
}

impl TryFrom<&Element> for Vec<String> {
    type Error = GetError;
    fn try_from(value: &Element) -> Result<Self, Self::Error> {
        match value {
            A(E::PmlString(arr)) => Ok(arr.clone()),
            A(E::PmlBool(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlU8(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlU16(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlU32(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlU64(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlU128(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlI8(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlI16(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlI32(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlI64(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlI128(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlF32(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            A(E::PmlF64(arr)) => Ok(arr.iter().map(TS::to_string).collect()),
            _ => Err(Self::Error::InvalidType)
        }
    }
}

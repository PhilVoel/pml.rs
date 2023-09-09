use crate::{Element, elem::ArrayElement as E};
use Element::PmlArray as A;

impl From<String> for Element {
    fn from(s: String) -> Self {
        Element::PmlString(s)
    }
}

impl From<&Element> for String {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlString(s) => s.clone(),
            Element::PmlBool(b) => b.to_string(),
            Element::PmlI8(i) => i.to_string(),
            Element::PmlI16(i) => i.to_string(),
            Element::PmlI32(i) => i.to_string(),
            Element::PmlI64(i) => i.to_string(),
            Element::PmlI128(i) => i.to_string(),
            Element::PmlU8(i) => i.to_string(),
            Element::PmlU16(i) => i.to_string(),
            Element::PmlU32(i) => i.to_string(),
            Element::PmlU64(i) => i.to_string(),
            Element::PmlU128(i) => i.to_string(),
            Element::PmlF32(f) => f.to_string(),
            Element::PmlF64(f) => f.to_string(),
            Element::PmlStruct(_) |
            Element::PmlArray(_) => panic!("Invalid type")
        }
    }
}

impl From<&Element> for Vec<String> {
    fn from(value: &Element) -> Self {
        match value {
            A(E::PmlString(arr)) => arr.clone(),
            A(E::PmlBool(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlU8(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlU16(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlU32(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlU64(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlU128(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlI8(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlI16(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlI32(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlI64(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlI128(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlF32(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            A(E::PmlF64(arr)) => arr.iter().map(|e| e.to_string()).collect(),
            _ => panic!("Not a string array")
        }
    }
}

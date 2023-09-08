use std::fmt::Display;
use crate::{Element, elem::ArrayElement, PmlStruct};

impl Display for ArrayElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl From<Vec<bool>> for Element {
    fn from(value: Vec<bool>) -> Self {
        Self::PmlArray(ArrayElement::ABool(value))
    }
}

impl From<Vec<PmlStruct>> for Element {
    fn from(value: Vec<PmlStruct>) -> Self {
        Self::PmlArray(ArrayElement::APmlStruct(value))
    }
}

impl From<Vec<String>> for Element {
    fn from(value: Vec<String>) -> Self {
        Self::PmlArray(ArrayElement::AString(value))
    }
}

impl From<Vec<f32>> for Element {
    fn from(value: Vec<f32>) -> Self {
        Self::PmlArray(ArrayElement::Af32(value))
    }
}

impl From<Vec<f64>> for Element {
    fn from(value: Vec<f64>) -> Self {
        Self::PmlArray(ArrayElement::Af64(value))
    }
}

impl From<Vec<i8>> for Element {
    fn from(value: Vec<i8>) -> Self {
        Self::PmlArray(ArrayElement::Ai8(value))
    }
}

impl From<Vec<i16>> for Element {
    fn from(value: Vec<i16>) -> Self {
        Self::PmlArray(ArrayElement::Ai16(value))
    }
}

impl From<Vec<i32>> for Element {
    fn from(value: Vec<i32>) -> Self {
        Self::PmlArray(ArrayElement::Ai32(value))
    }
}

impl From<Vec<i64>> for Element {
    fn from(value: Vec<i64>) -> Self {
        Self::PmlArray(ArrayElement::Ai64(value))
    }
}

impl From<Vec<i128>> for Element {
    fn from(value: Vec<i128>) -> Self {
        Self::PmlArray(ArrayElement::Ai128(value))
    }
}

impl From<Vec<u8>> for Element {
    fn from(value: Vec<u8>) -> Self {
        Self::PmlArray(ArrayElement::Au8(value))
    }
}

impl From<Vec<u16>> for Element {
    fn from(value: Vec<u16>) -> Self {
        Self::PmlArray(ArrayElement::Au16(value))
    }
}

impl From<Vec<u32>> for Element {
    fn from(value: Vec<u32>) -> Self {
        Self::PmlArray(ArrayElement::Au32(value))
    }
}

impl From<Vec<u64>> for Element {
    fn from(value: Vec<u64>) -> Self {
        Self::PmlArray(ArrayElement::Au64(value))
    }
}

impl From<Vec<u128>> for Element {
    fn from(value: Vec<u128>) -> Self {
        Self::PmlArray(ArrayElement::Au128(value))
    }
}

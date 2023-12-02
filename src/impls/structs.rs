use crate::{Element, PmlStruct, parse::WIPStruct, elem::ArrayElement, GetError};
use std::collections::HashMap;

impl From<HashMap<String, Element>> for Element {
    fn from(elements: HashMap<String, Element>) -> Self {
        Element::PmlStruct(Box::new(PmlStruct{elements}))
    }
}

impl From<WIPStruct> for PmlStruct {
    fn from(value: WIPStruct) -> Self {
        Self {
            elements: value.finished_elements
        }
    }
}

impl TryFrom<&Element> for Vec<PmlStruct> {
    type Error = GetError;
    fn try_from(value: &Element) -> Result<Self, Self::Error> {
        match value {
            Element::PmlArray(ArrayElement::PmlStruct(e)) => Ok(e.clone()),
            _ => Err(GetError::InvalidType)
        }
    }
}

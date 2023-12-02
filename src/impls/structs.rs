use crate::{Element, PmlStruct, elem::ArrayElement, GetError};
use std::collections::HashMap;

impl From<HashMap<String, Element>> for Element {
    fn from(elements: HashMap<String, Element>) -> Self {
        Element::PmlStruct(Box::new(PmlStruct{elements}))
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

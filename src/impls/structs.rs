use crate::{Element, PmlStruct, parse::WIPStruct, elem::ArrayElement};
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

impl From<&Element> for Vec<PmlStruct> {
    fn from(value: &Element) -> Self {
        match value {
            Element::PmlArray(ArrayElement::PmlStruct(arr)) => arr.clone(),
            _ => panic!("Invalid type")
        }
    }
}

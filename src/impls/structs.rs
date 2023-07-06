use crate::{Element, PmlStruct};
use std::collections::HashMap;

impl From<HashMap<String, Element>> for Element {
    fn from(elements: HashMap<String, Element>) -> Self {
        Element::PmlStruct(Box::new(PmlStruct{elements}))
    }
}

impl<'a> From<&'a Element> for &'a Box<PmlStruct> {
    fn from(e: &'a Element) -> Self {
        if let Element::PmlStruct(s) = e {
            s
        }
        else {
            todo!()
        }
    }
}

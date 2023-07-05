use crate::{Element, PmlStruct};
use std::collections::HashMap;

impl From<(HashMap<String, Element>, Option<Element>)> for Element {
    fn from(elements: (HashMap<String, Element>, Option<Element>)) -> Self {
        Element::PmlStruct(Box::new(PmlStruct{
            elements: elements.0,
            own_val: elements.1
        }))
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

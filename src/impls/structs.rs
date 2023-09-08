use crate::{Element, PmlStruct, parse::WIPStruct};
use std::{collections::HashMap, fmt::{Formatter, Result, Display}};

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

impl<'a> From<&'a Element> for &'a Box<PmlStruct> {
    fn from(e: &'a Element) -> Self {
        if let Element::PmlStruct(s) = e {
            s
        }
        else {
            panic!("Not a struct");
        }
    }
}

impl Display for PmlStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:#?}", self.elements)
    }
}

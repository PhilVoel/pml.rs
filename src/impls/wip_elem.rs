use std::{rc::Rc, cell::RefCell};
use crate::{parse::{WIPElement, ISElem, WIPStruct}, elem::Element};

impl<T> From<T> for WIPElement
where Element: From<T> {
    fn from(value: T) -> Self {
        Self::Element(value.into())
    }
}

impl From<Vec<ISElem>> for WIPElement {
    fn from(value: Vec<ISElem>) -> Self {
        Self::IncompleteString(value)
    }
}

impl From<Vec<(usize, Vec<ISElem>)>> for WIPElement {
    fn from(value: Vec<(usize, Vec<ISElem>)>) -> Self {
        Self::StringArray(value)
    }
}

impl From<Rc<RefCell<WIPStruct>>> for WIPElement {
    fn from(value: Rc<RefCell<WIPStruct>>) -> Self {
        Self::Struct(value)
    }
}

impl From<Vec<(usize, Rc<RefCell<WIPStruct>>)>> for WIPElement {
    fn from(value: Vec<(usize, Rc<RefCell<WIPStruct>>)>) -> Self {
        Self::StructArray(value)
    }
}

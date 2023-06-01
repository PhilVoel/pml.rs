use crate::{Element, parse::StringState};

impl From<Vec<(String, StringState)>> for Element {
    fn from(vec: Vec<(String, StringState)>) -> Self {
        Element::IncompleteString(vec)
    }
}

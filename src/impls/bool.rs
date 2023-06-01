use crate::Element;

impl<'a> From<&'a Element> for &'a bool {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlBool(b) => b,
            _ => panic!("Not a bool")
        }
    }
}

impl From<&Element> for bool {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlBool(b) => *b,
            _ => panic!("Not a bool")
        }
    }
}

impl From<bool> for Element {
    fn from(b: bool) -> Self {
        Element::PmlBool(b)
    }
}

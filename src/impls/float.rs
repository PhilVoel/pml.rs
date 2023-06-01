use crate::Element;

impl<'a> From<&'a Element> for &'a f64 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlF64(f) => f,
            _ => panic!("Not a float")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for f64 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlF64(f) => *f,
            Element::PmlF32(f) => *f as f64,
            _ => panic!("Not a float")
        }
    }
}

impl From<&Element> for f32 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlF32(f) => *f,
            _ => panic!("Not a float")
        }
    }
}

impl<'a> From<&'a Element> for &'a f32 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlF32(f) => f,
            _ => panic!("Not a float")
        }
    }
}

impl From<f64> for Element {
    fn from(f: f64) -> Self {
        Element::PmlF64(f)
    }
}

impl From<f32> for Element {
    fn from(f: f32) -> Self {
        Element::PmlF32(f)
    }
}

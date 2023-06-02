use crate::Element;


impl<'a> From<&'a Element> for &'a i128 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlI128(i) => i,
            _ => panic!("Not an int")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for i128 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlI128(i) => *i,
            Element::PmlI64(i) => *i as Self,
            Element::PmlI32(i) => *i as Self,
            Element::PmlI16(i) => *i as Self,
            Element::PmlI8(i) => *i as Self,
            Element::PmlU64(u) => *u as Self,
            Element::PmlU32(u) => *u as Self,
            Element::PmlU16(u) => *u as Self,
            Element::PmlU8(u) => *u as Self,
            _ => panic!("Not an int")
        }
    }
}

impl<'a> From<&'a Element> for &'a i64 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlI64(i) => i,
            _ => panic!("Not an int")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for i64 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlI64(i) => *i,
            Element::PmlI32(i) => *i as Self,
            Element::PmlI16(i) => *i as Self,
            Element::PmlI8(i) => *i as Self,
            Element::PmlU32(u) => *u as Self,
            Element::PmlU16(u) => *u as Self,
            Element::PmlU8(u) => *u as Self,
            _ => panic!("Not an int")
        }
    }
}

impl<'a> From<&'a Element> for &'a i32 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlI32(i) => i,
            _ => panic!("Not an int")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for i32 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlI32(i) => *i,
            Element::PmlI16(i) => *i as Self,
            Element::PmlI8(i) => *i as Self,
            Element::PmlU16(u) => *u as Self,
            Element::PmlU8(u) => *u as Self,
            _ => panic!("Not an int")
        }
    }
}

impl<'a> From<&'a Element> for &'a i16 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlI16(i) => i,
            _ => panic!("Not an int")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for i16 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlI16(i) => *i,
            Element::PmlI8(i) => *i as Self,
            Element::PmlU8(u) => *u as Self,
            _ => panic!("Not an int")
        }
    }
}

impl<'a> From<&'a Element> for &'a i8 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlI8(i) => i,
            _ => panic!("Not an int")
        }
    }
}

impl From<&Element> for i8 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlI8(i) => *i,
            _ => panic!("Not an int")
        }
    }
}

impl From<i128> for Element {
    fn from(i: i128) -> Self {
        Element::PmlI128(i)
    }
}

impl From<i64> for Element {
    fn from(i: i64) -> Self {
        Element::PmlI64(i)
    }
}

impl From<i32> for Element {
    fn from(i: i32) -> Self {
        Element::PmlI32(i)
    }
}

impl From<i16> for Element {
    fn from(i: i16) -> Self {
        Element::PmlI16(i)
    }
}

impl From<i8> for Element {
    fn from(i: i8) -> Self {
        Element::PmlI8(i)
    }
}

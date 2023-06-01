use crate::Element;

#[allow(clippy::cast_lossless)]

impl<'a> From<&'a Element> for &'a u128 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlU128(i) => i,
            _ => panic!("Not an unsigned int")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for u128 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlU128(u) => *u,
            Element::PmlU64(u) => *u as Self,
            Element::PmlU32(u) => *u as Self,
            Element::PmlU16(u) => *u as Self,
            Element::PmlU8(u) => *u as Self,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl<'a> From<&'a Element> for &'a u64 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlU64(i) => i,
            _ => panic!("Not an unsigned int")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for u64 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlU64(u) => *u,
            Element::PmlU32(u) => *u as Self,
            Element::PmlU16(u) => *u as Self,
            Element::PmlU8(u) => *u as Self,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl<'a> From<&'a Element> for &'a u32 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlU32(i) => i,
            _ => panic!("Not an unsigned int")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for u32 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlU32(u) => *u,
            Element::PmlU16(u) => *u as Self,
            Element::PmlU8(u) => *u as Self,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl<'a> From<&'a Element> for &'a u16 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlU16(i) => i,
            _ => panic!("Not an unsigned int")
        }
    }
}

#[allow(clippy::cast_lossless)]
impl From<&Element> for u16 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlU16(u) => *u,
            Element::PmlU8(u) => *u as Self,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl<'a> From<&'a Element> for &'a u8 {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlU8(i) => i,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl From<&Element> for u8 {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlU8(u) => *u,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl From<u128> for Element {
    fn from(u: u128) -> Self {
        Element::PmlU128(u)
    }
}

impl From<u64> for Element {
    fn from(u: u64) -> Self {
        Element::PmlU64(u)
    }
}

impl From<u32> for Element {
    fn from(u: u32) -> Self {
        Element::PmlU32(u)
    }
}

impl From<u16> for Element {
    fn from(u: u16) -> Self {
        Element::PmlU16(u)
    }
}

impl From<u8> for Element {
    fn from(u: u8) -> Self {
        Element::PmlU8(u)
    }
}

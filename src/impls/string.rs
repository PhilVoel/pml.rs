use crate::Element;

impl<'a> From<&'a Element> for &'a String {
    fn from(elem: &'a Element) -> Self {
        match elem {
            Element::PmlString(s) => s,
            _ => panic!("Not a string")
        }
    }
}

impl From<&Element> for String {
    fn from(elem: &Element) -> Self {
        match elem {
            Element::PmlString(s) => s.clone(),
            Element::PmlBool(b) => b.to_string(),
            Element::PmlI8(i) => i.to_string(),
            Element::PmlI16(i) => i.to_string(),
            Element::PmlI32(i) => i.to_string(),
            Element::PmlI64(i) => i.to_string(),
            Element::PmlI128(i) => i.to_string(),
            Element::PmlU8(i) => i.to_string(),
            Element::PmlU16(i) => i.to_string(),
            Element::PmlU32(i) => i.to_string(),
            Element::PmlU64(i) => i.to_string(),
            Element::PmlU128(i) => i.to_string(),
            Element::PmlF32(f) => f.to_string(),
            Element::PmlF64(f) => f.to_string(),
            Element::PmlStruct(s) => s.to_string(),
            Element::PmlArray(a) => a.to_string(),
        }
    }
}

impl From<String> for Element {
    fn from(s: String) -> Self {
        Element::PmlString(s)
    }
}

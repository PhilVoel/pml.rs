use crate::PmlElem;

impl<'a> From<&'a PmlElem> for &'a u64 {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlUnsigned(u) => u,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl Into<PmlElem> for u64 {
    fn into(self) -> PmlElem {
        PmlElem::PmlUnsigned(self)
    }
}

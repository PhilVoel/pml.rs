use crate::PmlElem;

impl<'a> From<&'a PmlElem> for &'a u128 {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlUnsigned(u) => u,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl Into<PmlElem> for u128 {
    fn into(self) -> PmlElem {
        PmlElem::PmlUnsigned(self)
    }
}

impl Into<PmlElem> for u64 {
    fn into(self) -> PmlElem {
        PmlElem::PmlUnsigned(self.into())
    }
}

impl Into<PmlElem> for u32 {
    fn into(self) -> PmlElem {
        PmlElem::PmlUnsigned(self.into())
    }
}

impl Into<PmlElem> for u16 {
    fn into(self) -> PmlElem {
        PmlElem::PmlUnsigned(self.into())
    }
}

impl Into<PmlElem> for u8 {
    fn into(self) -> PmlElem {
        PmlElem::PmlUnsigned(self.into())
    }
}

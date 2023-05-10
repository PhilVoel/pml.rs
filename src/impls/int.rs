use crate::PmlElem;

impl<'a> From<&'a PmlElem> for &'a i128 {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlInt(i) => i,
            _ => panic!("Not an int")
        }
    }
}

impl Into<PmlElem> for i128 {
    fn into(self) -> PmlElem {
        PmlElem::PmlInt(self)
    }
}

impl Into<PmlElem> for i64 {
    fn into(self) -> PmlElem {
        PmlElem::PmlInt(self.into())
    }
}

impl Into<PmlElem> for i32 {
    fn into(self) -> PmlElem {
        PmlElem::PmlInt(self.into())
    }
}

impl Into<PmlElem> for i16 {
    fn into(self) -> PmlElem {
        PmlElem::PmlInt(self.into())
    }
}

impl Into<PmlElem> for i8 {
    fn into(self) -> PmlElem {
        PmlElem::PmlInt(self.into())
    }
}

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

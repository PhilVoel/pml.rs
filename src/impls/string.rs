use crate::PmlElem;

impl<'a> From<&'a PmlElem> for &'a String {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlString(s) => s,
            _ => panic!("Not a string")
        }
    }
}

impl Into<PmlElem> for String {
    fn into(self) -> PmlElem {
        PmlElem::PmlString(self)
    }
}

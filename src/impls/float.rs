use crate::PmlElem;

impl<'a> From<&'a PmlElem> for &'a f64 {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlFloat(f) => f,
            _ => panic!("Not a float")
        }
    }
}

impl Into<PmlElem> for f64 {
    fn into(self) -> PmlElem {
        PmlElem::PmlFloat(self)
    }
}

impl Into<PmlElem> for f32 {
    fn into(self) -> PmlElem {
        PmlElem::PmlFloat(self.into())
    }
}

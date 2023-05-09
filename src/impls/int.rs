use crate::PmlElem;

impl<'a> From<&'a PmlElem> for &'a i64 {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlInt(i) => i,
            _ => panic!("Not an int")
        }
    }
}

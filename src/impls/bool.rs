use crate::PmlElem;

impl<'a> From<&'a PmlElem> for &'a bool {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlBool(b) => b,
            _ => panic!("Not a bool")
        }
    }
}

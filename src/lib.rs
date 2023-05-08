use std::collections::HashMap;

mod parse;
pub use parse::parse_file;

mod elem {
    pub enum PmlElem {
        PmlString(String),
        PmlInt(i64),
        PmlUnsigned(u64),
        PmlFloat(f64), 
        PmlBool(bool)
    }
}

use elem::PmlElem;
pub struct PmlStruct {
    elements: Option<HashMap<String, PmlElem>>
}



impl<'a> PmlStruct {
    pub fn get<'b, T>(&'a self, key: &'b str) -> &'a T
        where
        &'a T: From<&'a PmlElem>
        {
            self.elements.as_ref().unwrap().get(key).unwrap().try_into().unwrap()
        }
}

impl<'a> From<&'a PmlElem> for &'a String {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlString(s) => s,
            _ => panic!("Not a string")
        }
    }
}

impl<'a> From<&'a PmlElem> for &'a bool {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlBool(b) => b,
            _ => panic!("Not a bool")
        }
    }
}

impl<'a> From<&'a PmlElem> for &'a i64 {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlInt(i) => i,
            _ => panic!("Not an int")
        }
    }
}

impl<'a> From<&'a PmlElem> for &'a u64 {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlUnsigned(u) => u,
            _ => panic!("Not an unsigned int")
        }
    }
}

impl<'a> From<&'a PmlElem> for &'a f64 {
    fn from(elem: &'a PmlElem) -> Self {
        match elem {
            PmlElem::PmlFloat(f) => f,
            _ => panic!("Not a float")
        }
    }
}

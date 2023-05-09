use std::collections::HashMap;

mod impls;
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

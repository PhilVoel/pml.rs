use std::collections::HashMap;

mod parse;
pub use parse::parse_file;

pub struct PmlStruct {
    elements: Option<HashMap<String, PmlElem>>
}

enum PmlElem {
    PmlString(String),
    PmlInt(i64),
    PmlUnsigned(u64),
    PmlFloat(f64), 
    PmlBool(bool)
}

impl PmlStruct {
    pub fn get_string(&self, key: &str) -> &str {
        match self.elements.as_ref().unwrap().get(key) {
            Some(PmlElem::PmlString(s)) => s,
            _ => panic!("Not a string")
        }
    }

    pub fn get_bool(&self, key: &str) -> &bool {
        match self.elements.as_ref().unwrap().get(key) {
            Some(PmlElem::PmlBool(b)) => b,
            _ => panic!("Not a bool")
        }
    }

    pub fn get_unsigned(&self, key: &str) -> &u64 {
        match self.elements.as_ref().unwrap().get(key) {
            Some(PmlElem::PmlUnsigned(i)) => i,
            _ => panic!("Not an unsigned")
        }
    }

    pub fn get_int(&self, key: &str) -> &i64 {
        match self.elements.as_ref().unwrap().get(key) {
            Some(PmlElem::PmlInt(i)) => i,
            _ => panic!("Not an int")
        }
    }

    pub fn get_float(&self, key: &str) -> &f64 {
        match self.elements.as_ref().unwrap().get(key) {
            Some(PmlElem::PmlFloat(f)) => f,
            _ => panic!("Not a float")
        }
    }
}

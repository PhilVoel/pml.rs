use std::fs;
use std::collections::HashMap;

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

pub fn parse_file(file: &str) -> PmlStruct {
    let lines = get_lines(String::from(file) + ".pml");
    parse_lines(lines)
}

fn get_lines(file: String) -> Vec<String> {
    let mut lines = Vec::new();
    let contents = fs::read_to_string(file).unwrap();
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    lines
}

fn parse_lines(lines: Vec<String>) -> PmlStruct {
    let mut elements_map: HashMap<String, PmlElem> = HashMap::new();
    for line in lines {
        let (key, value) = line.split_once("=").unwrap();
        if let Ok(num) = value.parse::<u64>() {
            elements_map.insert(key.to_string(), PmlElem::PmlUnsigned(num));
        } else if let Ok(num) = value.parse::<i64>() {
            elements_map.insert(key.to_string(), PmlElem::PmlInt(num));
        } else if let Ok(num) = value.parse::<f64>() {
            elements_map.insert(key.to_string(), PmlElem::PmlFloat(num));
        } else if let Ok(bool) = value.parse::<bool>() {
            elements_map.insert(key.to_string(), PmlElem::PmlBool(bool));
        } else {
            elements_map.insert(key.to_string(), PmlElem::PmlString(value.to_string()));
        }
    }
    PmlStruct {elements: Some(elements_map)}
}

pub const fn new() -> PmlStruct {
    PmlStruct {elements: None}
}

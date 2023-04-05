use std::fs;
use std::collections::HashMap;

pub struct PmlStruct {
    elements: HashMap<String, PmlElem>
}

enum PmlElem {
    PmlString(String),
    PmlInt(i32),
    PmlFloat(f32)
}

impl PmlStruct {
    pub fn get_string(&self, key: &str) -> &str {
        match self.elements.get(key) {
            Some(PmlElem::PmlString(s)) => s,
            _ => panic!("Not a string")
        }
    }

    pub fn get_int(&self, key: &str) -> &i32 {
        match self.elements.get(key) {
            Some(PmlElem::PmlInt(i)) => i,
            _ => panic!("Not an int")
        }
    }

    pub fn get_float(&self, key: &str) -> &f32 {
        match self.elements.get(key) {
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
    let mut elements: HashMap<String, PmlElem> = HashMap::new();
    for line in lines {
        let (key, value) = line.split_once("=").unwrap();
        if let Ok(num) = value.parse::<i32>() {
            elements.insert(key.to_string(), PmlElem::PmlInt(num));
        } else if let Ok(num) = value.parse::<f32>() {
            elements.insert(key.to_string(), PmlElem::PmlFloat(num));
        } else {
            elements.insert(key.to_string(), PmlElem::PmlString(value.to_string()));
        }
    }
    PmlStruct {elements}
}

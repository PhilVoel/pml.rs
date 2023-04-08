use std::fs;
use std::collections::HashMap;
use crate::{PmlElem, PmlStruct};

mod check_type;

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

pub fn parse_lines(lines: Vec<String>) -> PmlStruct {
    let mut elements_map: HashMap<String, PmlElem> = HashMap::new();
    for line in lines {
        let (key, value) = line.split_once("=").unwrap();
        let key = key.trim();
        let value = value.trim();
        if check_type::string(value) {
            elements_map.insert(key.to_string(), PmlElem::PmlString(value[1..value.len()-1].replace("\\\"", "\"").to_string()));
        } else if check_type::int(value) {
            elements_map.insert(key.to_string(), PmlElem::PmlInt(value.parse::<i64>().unwrap()));
        } else if check_type::unsigned(value) {
            elements_map.insert(key.to_string(), PmlElem::PmlUnsigned(value.parse::<u64>().unwrap()));
        } else if check_type::float(value) {
            elements_map.insert(key.to_string(), PmlElem::PmlFloat(value.parse::<f64>().unwrap()));
        } else if check_type::bool(value) {
            elements_map.insert(key.to_string(), PmlElem::PmlBool(value.parse::<bool>().unwrap()));
        }
    }
    PmlStruct {elements: Some(elements_map)}
}

use std::fs;
use std::collections::HashMap;
use crate::{Element, PmlStruct, Error};

mod string_state {
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum StringState {
        Text,
        Variable,
        None
    }
}
pub (crate) use string_state::StringState;

pub fn file(file: &str) -> Result<PmlStruct, Error> {
    parse_lines(
        get_lines(String::from(file))?
    )
}

fn get_lines(file: String) -> Result<Vec<String>, Error> {
    let mut lines = Vec::new();
    let contents = fs::read_to_string(file)?;
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    Ok(lines)
}

fn parse_lines(lines: Vec<String>) -> Result<PmlStruct, Error> {
    let mut elements: HashMap<String, Element> = HashMap::new();
    let mut incomplete_strings: HashMap<String, Vec<(String, StringState)>> = HashMap::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {return Err(Error::Parse)};
        let key = key.trim().to_string();
        let elem = convert_to_pmlelem(value.trim(), key.clone())?;
        match elem {
            Element::IncompleteString(vec) => {incomplete_strings.insert(key.clone(), vec);},
            elem => if let Some(old_val) = elements.insert(key.clone(), elem.clone()) {
                return Err(Error::AlreadyExists{key, old_val, new_val:elem});
            }
        }
    }
    for (name, inc_str) in &incomplete_strings {
        let mut names = vec![name];
        if check_circular_depedencies(&mut names, inc_str , &incomplete_strings) {
            return Err(Error::CircularDependency(names.iter().map(|s| (*s).to_string()).collect()));
        }
    }
    while !incomplete_strings.is_empty() {
        let mut incomplete_strings_2: HashMap<String, Vec<(String, StringState)>> = HashMap::new();
        for (key, inc_str) in incomplete_strings {
            let mut accum_str = String::new();
            let mut split: Vec<(String, StringState)> = Vec::new();
            for (value, state) in inc_str {
                match state {
                    StringState::Text => accum_str.push_str(&value),
                    StringState::Variable => {
                        if let Some(val) = elements.get(&value) {
                            accum_str.push_str(&val.to_string());
                        } else {
                            split.push((accum_str, StringState::Text));
                            accum_str = String::new();
                            split.push((value, StringState::Variable));
                        }
                    },
                    StringState::None => ()
                }
            }
            if split.is_empty() {
                elements.insert(key, accum_str.into());
            }
            else {
                split.push((accum_str, StringState::Text));
                incomplete_strings_2.insert(key, split);
            }
        }
        incomplete_strings = incomplete_strings_2;
    }
    Ok(PmlStruct {elements})
}

fn check_circular_depedencies<'a>(names: &mut Vec<&'a String>, dependencies: &'a [(String, StringState)], incomplete_strings: &'a HashMap<String, Vec<(String, StringState)>>) -> bool {
    let dependencies: Vec<&String> = dependencies.iter().filter(|(_,state)| *state==StringState::Variable).map(|(val,_)| val).collect();
    for dependency in dependencies {
        if names.contains(&dependency) {
            return true;
        }
        match incomplete_strings.get(dependency) {
            None => (),
            Some(vec) => {
                names.push(dependency);
                if check_circular_depedencies(names, vec, incomplete_strings) {
                    return true;
                }
            }
        }
    }
    false
}

fn convert_to_pmlelem(value: &str, key: String) -> Result<Element, Error> {
    //String
    if value.starts_with('"') || value.starts_with('{') && value.ends_with('"') || value.ends_with('}') {
        use StringState::*;
        let value = value.replace("\\\"", "\"").replace("\\n", "\n");
        let mut to_insert = String::new();
        let mut state = None;
        let mut split: Vec<(String, StringState)> = Vec::new();
        for c in value.chars() {
            match (state, c) {
                (None, '"') => state = Text,
                (None, '{') => state = Variable,
                (None, ' ') => (),
                (None, _) => return Err(Error::Parse),
                (Text, '"') if !to_insert.ends_with('\\') => {
                    state = None;
                    if !to_insert.is_empty() {
                        split.push((to_insert.clone(), Text));
                        to_insert.clear();
                    }
                },
                (Variable, '}') => {
                    state = None;
                    if !to_insert.is_empty() {
                        split.push((to_insert.clone(), Variable));
                        to_insert.clear();
                    }
                },
                (Variable, ' ') => return Err(Error::Parse),
                (Text|Variable, _) => to_insert.push(c),
            }
        }
        if state != None {
            return Err(Error::Parse);
        }
        Ok(split.into())
    //Bool
    } else if value == "true" {
        Ok(true.into())
    } else if value == "false" {
        Ok(false.into())
    //Number
    } else if let Some(stripped) = value.strip_prefix('(') {
        let (force_type, val) = stripped.split_once(')').ok_or(Error::Parse)?;
        match force_type.trim() {
            "s8" => Ok(val.trim().parse::<i8>()?.into()),
            "s16" => Ok(val.trim().parse::<i16>()?.into()),
            "s32" => Ok(val.trim().parse::<i32>()?.into()),
            "s64" => Ok(val.trim().parse::<i64>()?.into()),
            "s128" => Ok(val.trim().parse::<i128>()?.into()),
            "u8" => Ok(val.trim().parse::<u8>()?.into()),
            "u16" => Ok(val.trim().parse::<u16>()?.into()),
            "u32" => Ok(val.trim().parse::<u32>()?.into()),
            "u64" => Ok(val.trim().parse::<u64>()?.into()),
            "u128" => Ok(val.trim().parse::<u128>()?.into()),
            "f32" => Ok(val.trim().parse::<f32>()?.into()),
            "f64" => Ok(val.trim().parse::<f64>()?.into()),
            type_name => Err(Error::UnknownTypeForced{key, type_name:type_name.to_string()})
        }
    } else {
        match value.parse::<i128>() {
            Ok(num) => {
                if num < 0 {
                    #[allow(clippy::cast_possible_truncation)]
                    match num {
                        -128..=-1 => Ok((num as i8).into()),
                        -32_768..=-129 => Ok((num as i16).into()),
                        -2_147_483_648..=-32_769 => Ok((num as i32).into()),
                        -9_223_372_036_854_775_808..=-2_147_483_649 => Ok((num as i64).into()),
                        _ => Ok(num.into())
                    }
                }
                else {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    match num {
                        0..=255 => Ok((num as u8).into()),
                        256..=65_535 => Ok((num as u16).into()),
                        65_536..=4_294_967_295 => Ok((num as u32).into()),
                        _ => Ok((num as u64).into()),
                    }
                }
            },
            Err(_) => match value.parse::<u128>() {
                Ok(num) => Ok((num).into()),
                Err(_) => match value.parse::<f64>() {
                    Ok(num64) => match value.parse::<f32>() {
                        Ok(num32) => Ok(num32.into()),
                        Err(_) => Ok(num64.into())
                    }
                    Err(_) => Err(Error::Parse)
                }
            }
        }
    }
}

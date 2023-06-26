use std::fs;
use std::collections::{HashMap, HashSet};
use crate::{Element, PmlStruct, Error, ParseNumberError};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TextState {
    Literal,
    LiteralEscaped,
    VariableStart,
    Variable,
    VariableDone,
    Between
}

#[derive(PartialEq, Debug)]
enum ParseState {
    KeyStart,
    Key,
    KeyDone,
    ValueStart,
    ValueForceStart,
    ValueForce,
    ValueForceDone,
    ValueAllowSpace(ForcedNumberCategory),
    Value(ValueType),
    ValueDone
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum ValueType {
    Text(TextState),
    Bool,
    Number(NumberType),
    Forced(ForcedNumberCategory)
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum NumberType {
    Signed,
    Unsigned,
    Decimal
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ForcedNumberCategory {
    Signed(ForcedSigned),
    Unsigned(ForcedUnsigned),
    Decimal(ForcedDecimal)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ForcedDecimal {
    F32(bool, bool),
    F64(bool, bool)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ForcedSigned {
    I8(bool),
    I16(bool),
    I32(bool),
    I64(bool),
    I128(bool)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ForcedUnsigned {
    U8,
    U16,
    U32,
    U64,
    U128
}

pub fn file(file: &str) -> Result<PmlStruct, Error> {
    let file_content = fs::read_to_string(file)?;
    parse_string(&file_content)
}

fn parse_string(string: &str) -> Result<PmlStruct, Error> {
    use ParseState::{KeyStart, Key, KeyDone, ValueStart, ValueForceStart, ValueForce, ValueForceDone, ValueAllowSpace, Value, ValueDone};
    use ValueType::{Text, Bool, Number, Forced};
    use TextState::{Literal, LiteralEscaped, VariableStart, Variable, VariableDone, Between};
    use NumberType::{Signed, Unsigned, Decimal};
    use ForcedNumberCategory as FNC;
    use ForcedDecimal::{F32, F64};
    use ForcedSigned::{I8, I16, I32, I64, I128};
    use ForcedUnsigned::{U8, U16, U32, U64, U128};

    let mut elements: HashMap<String, Element> = HashMap::new();
    let mut string_elements: Vec<(TextState, String)> = Vec::new();
    let mut incomplete_strings: HashMap<String, Vec<(TextState, String)>> = HashMap::new();
    let mut state = KeyStart;
    let mut key = String::new();
    let mut value = String::new();
    let mut force = String::new();
    let mut line_counter: u32 = 1;
    let mut column_counter: u32 = 0;

    for current_char in string.chars() {
        column_counter += 1;
        match (&state, current_char) {
            (KeyStart, c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            },
            (KeyStart, c) if is_char_reserved(c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (KeyStart, c) => {
                state = Key;
                key.push(c);
            }
            (Key, '=') => match elements.get(&key) {
                None => state = ValueStart,
                Some(old_val) => return Err(Error::AlreadyExists{key, old_val: old_val.clone(), line: line_counter})
            }
            (Key, c) if c.is_whitespace() => match elements.get(&key) {
                None => {
                    state = KeyDone;
                    if c == '\n' {
                        line_counter += 1;
                        column_counter = 0;
                    }
                }
                Some(old_val) => return Err(Error::AlreadyExists{key, old_val: old_val.clone(), line: line_counter})
            }
            (Key, c) if is_char_reserved(c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (Key, c) => key.push(c),
            (KeyDone, '=') => state = ValueStart,
            (KeyDone, c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            },
            (KeyDone, c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (ValueStart, c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (ValueStart, '(') => state = ValueForceStart,
            (ValueStart, '{') => state = Value(Text(Variable)),
            (ValueStart, '"') => state = Value(Text(Literal)),
            (ValueStart, '-') => {
                value.push('-');
                state = Value(Number(Signed));
            }
            (ValueStart, '.') => {
                value.push('.');
                state = Value(Number(Decimal));
            }
            (ValueStart, c) if is_char_reserved(c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (ValueStart, c) if c.is_ascii_digit() => {
                state = Value(Number(Unsigned));
                value.push(c);
            }
            (ValueStart, c @ ('t'|'f')) => {
                state = Value(Bool);
                value.push(c);
            }
            (ValueStart, c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (ValueForceStart, c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (ValueForceStart, c) if is_char_reserved(c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (ValueForceStart, c) => {
                state = ValueForce;
                force.push(c);
            }
            (ValueForce, ')') => {
                let force_type = match force.as_str() {
                    "u8" => FNC::Unsigned(U8),
                    "u16" => FNC::Unsigned(U16),
                    "u32" => FNC::Unsigned(U32),
                    "u64" => FNC::Unsigned(U64),
                    "u128" => FNC::Unsigned(U128),
                    "s8" => FNC::Signed(I8(false)),
                    "s16" => FNC::Signed(I16(false)),
                    "s32" => FNC::Signed(I32(false)),
                    "s64" => FNC::Signed(I64(false)),
                    "s128" => FNC::Signed(I128(false)),
                    "f32" => FNC::Decimal(F32(false, false)),
                    "f64" => FNC::Decimal(F64(false, false)),
                    t => return Err(Error::UnknownForcedType {
                        key,
                        type_name: t.to_string()
                    })
                };
                state = ValueAllowSpace(force_type);
            }
            (ValueForce, c) if c.is_whitespace() => {
                state= ValueForceDone;
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (ValueForce, c) if is_char_reserved(c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (ValueForce, c) => force.push(c),
            (ValueForceDone, ')') => {
                let force_type = match force.as_str() {
                    "u8" => FNC::Unsigned(U8),
                    "u16" => FNC::Unsigned(U16),
                    "u32" => FNC::Unsigned(U32),
                    "u64" => FNC::Unsigned(U64),
                    "u128" => FNC::Unsigned(U128),
                    "s8" => FNC::Signed(I8(false)),
                    "s16" => FNC::Signed(I16(false)),
                    "s32" => FNC::Signed(I32(false)),
                    "s64" => FNC::Signed(I64(false)),
                    "s128" => FNC::Signed(I128(false)),
                    "f32" => FNC::Decimal(F32(false, false)),
                    "f64" => FNC::Decimal(F64(false, false)),
                    t => return Err(Error::UnknownForcedType {
                        key,
                        type_name: t.to_string()
                    })
                };
                state = ValueAllowSpace(force_type);
                force = String::new();
            }
            (ValueForceDone, c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (ValueForceDone, c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (ValueAllowSpace(_), c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (ValueAllowSpace(f @ FNC::Decimal(_)), '.') => {
                value.push('.');
                state = Value(Forced(disable_decimal_point(*f)));
            }
            (ValueAllowSpace(_), '.') => return Err(Error::IllegalCharacter {
                char: '.',
                line: line_counter,
                col: column_counter
            }),
            (ValueAllowSpace(f), '-') => {
                value.push('-');
                state = match f {
                    FNC::Signed(_)|FNC::Decimal(_) => Value(Forced(disable_negative_sign(*f))),
                    FNC::Unsigned(_) => return Err(Error::IllegalCharacter {
                        char: '-',
                        line: line_counter,
                        col: column_counter
                    })
                }
            }
            (ValueAllowSpace(f), c) if c.is_ascii_digit() => {
                value.push(c);
                state = Value(Forced(disable_negative_sign(*f)));
            }
            (ValueAllowSpace(_), c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (Value(Bool), c) => match (value.as_str(), c) {
                #[allow(clippy::unnested_or_patterns)]
                ("t", 'r')|
                ("tr", 'u')|
                ("tru", 'e')|
                ("f", 'a')|
                ("fa", 'l')|
                ("fal", 's')|
                ("fals", 'e') => {
                    value.push(c);
                    if c == 'e' {
                        let elem = (value.as_str() == "true").into();
                        elements.insert(key, elem);
                        state = ValueDone;
                        key = String::new();
                        value = String::new();
                    }
                }
                _ => return Err(Error::IllegalCharacter {
                    char: c,
                    line: line_counter,
                    col: column_counter
                })
            }
            (Value(Number(_)), c) if c.is_ascii_digit() => value.push(c),
            (Value(Number(Unsigned|Signed)), '.') => {
                value.push('.');
                state = Value(Number(Decimal));
            }
            (Value(Number(_)), '_') => (),
            (Value(Number(t)), ';') => {
                let num = match get_number_from_string(*t, &value){
                    Ok(n) => n,
                    Err(e) => return Err(Error::ParseNumberError {
                        line: line_counter,
                        value,
                        error: e
                    })
                };
                elements.insert(key, num);
                key = String::new();
                value = String::new();
                state = KeyStart;
            }
            (Value(Number(t)), c) if c.is_whitespace() => {
                let num = match get_number_from_string(*t, &value){
                    Ok(n) => n,
                    Err(e) => return Err(Error::ParseNumberError {
                        line: line_counter,
                        value,
                        error: e
                    })
                };
                elements.insert(key, num);
                key = String::new();
                value = String::new();
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
                state = ValueDone;
            }
            (Value(Number(_)), c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (Value(Text(Literal)), '\\') => state = Value(Text(Literal)),
            (Value(Text(Literal)), '"') => {
                state = Value(Text(Between));
                string_elements.push((Literal, value));
                value = String::new();
            }
            (Value(Text(Literal)), c) => value.push(c),
            (Value(Text(LiteralEscaped)), c) => {
                state = Value(Text(Literal));
                value.push(match c {
                    'r' => continue,
                    't' => '\t',
                    'n' => '\n',
                    c => c
                });
            }
            (Value(Text(VariableStart)), c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (Value(Text(VariableStart)), c) if is_char_reserved(c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (Value(Text(VariableStart)), c) => {
                value.push(c);
                state = Value(Text(Variable));
            }
            (Value(Text(Variable)), '}') => {
                string_elements.push((Variable, value));
                value = String::new();
                state = Value(Text(Between));
            }
            (Value(Text(Variable)), c) if c.is_whitespace() => {
                state = Value(Text(VariableDone));
                string_elements.push((Variable, value));
                value = String::new();
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (Value(Text(Variable)), c) if is_char_reserved(c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (Value(Text(Variable)), c) => value.push(c),
            (Value(Text(VariableDone)), c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (Value(Text(VariableDone)), '}') => state = Value(Text(Between)),
            (Value(Text(VariableDone)), c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (Value(Text(Between)), '"') => state = Value(Text(Literal)),
            (Value(Text(Between)), '{') => state = Value(Text(VariableStart)),
            (Value(Text(Between)), c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter +=1;
                    column_counter = 0;
                }
            }
            (Value(Text(Between)), ';') => {
                state = KeyStart;
                incomplete_strings.insert(key, string_elements);
                string_elements = Vec::new();
                key = String::new();
            }
            (Value(Text(Between)), c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (Value(Forced(t@FNC::Decimal(F32(_, false)|F64(_, false)))), '.') => {
                state = Value(Forced(disable_decimal_point(*t)));
                value.push('.');
            }
            (Value(Forced(_)), '.') => return Err(Error::IllegalCharacter {
                char: '.',
                line: line_counter,
                col: column_counter
            }),
            (Value(Forced(t@(FNC::Signed(I8(false)|I16(false)|I32(false)|I64(false)|I128(false))|FNC::Decimal(F32(false, false)|F64(false, false))))), '-') => {
                value.push('-');
                state = Value(Forced(disable_negative_sign(*t)));
            }
            (Value(Forced(_)), '-') => return Err(Error::IllegalCharacter {
                char: '-',
                line: line_counter,
                col: column_counter
            }),
            (Value(Forced(_)), c) if c.is_ascii_digit() => value.push(c),
            (Value(Forced(f)), c) if c.is_whitespace() => {
                let num = match get_number_from_forced(*f, &value){
                    Ok(n) => n,
                    Err(e) => return Err(Error::ParseNumberError {
                        line: line_counter,
                        value,
                        error: e
                    })
                };
                elements.insert(key, num);
                key = String::new();
                value = String::new();
                state = ValueDone;
            }
            (Value(Forced(_)), c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            }),
            (ValueDone, c) if c.is_whitespace() => {
                if c == '\n' {
                    line_counter += 1;
                    column_counter = 0;
                }
            }
            (ValueDone, ';') => state = KeyStart,
            (ValueDone, c) => return Err(Error::IllegalCharacter {
                char: c,
                line: line_counter,
                col: column_counter
            })
        }
    }
    if state != KeyStart {
        return Err(Error::UnexpectedEOF);
    }
    for (name, inc_str) in &incomplete_strings {
        let mut names = HashSet::new();
        names.insert(name);
        let dependencies: HashSet<&String> = inc_str.iter().filter(|(state, _)| *state==TextState::Variable).map(|(_, val)| val).collect();
        for dependency in &dependencies {
            match elements.get(*dependency) {
                Some(_) => (),
                None => match incomplete_strings.get(*dependency) {
                    Some(_) => (),
                    None => return Err(Error::UnfulfilledDependency{key: String::from(name), dependency: String::from(*dependency)})
                }
            }
        }
        if check_circular_depedencies(&mut names, dependencies , &incomplete_strings) {
            return Err(Error::CircularDependency(names.iter().map(|s| (*s).to_string()).collect()));
        }
    }
    while !incomplete_strings.is_empty() {
        let mut incomplete_strings_2: HashMap<String, Vec<(TextState, String)>> = HashMap::new();
        for (key, inc_str) in incomplete_strings {
            let mut accum_str = String::new();
            let mut split: Vec<(TextState, String)> = Vec::new();
            for (state, value) in inc_str {
                match state {
                    TextState::Literal => accum_str.push_str(&value),
                    TextState::Variable => {
                        if let Some(val) = elements.get(&value) {
                            accum_str.push_str(&val.to_string());
                        } else {
                            split.push((TextState::Literal, accum_str));
                            accum_str = String::new();
                            split.push((TextState::Variable, value));
                        }
                    },
                    _ => ()
                }
            }
            if split.is_empty() {
                elements.insert(key, accum_str.into());
            }
            else {
                split.push((TextState::Literal, accum_str));
                incomplete_strings_2.insert(key, split);
            }
        }
        incomplete_strings = incomplete_strings_2;
    }
    Ok(PmlStruct {elements})
}

fn check_circular_depedencies<'a>(names: &mut HashSet<&'a String>, dependencies: HashSet<&'a String>, incomplete_strings: &'a HashMap<String, Vec<(TextState, String)>>) -> bool {
    for dependency in dependencies {
        if names.contains(&dependency) {
            return true;
        }
        match incomplete_strings.get(dependency) {
            None => (),
            Some(vec) => {
                names.insert(dependency);
                let dependencies: HashSet<&String> = vec.iter().filter(|(state, _)| *state==TextState::Variable).map(|(_, val)| val).collect();
                if check_circular_depedencies(names, dependencies, incomplete_strings) {
                    return true;
                }
            }
        }
    }
    false
}

fn is_char_reserved(c: char) -> bool {
    ['=', ';', ',', '{', '}', '(', ')', '"', '\n', '[', ']', ':', '|', '.'].into_iter().any(|r| r == c)
}

fn disable_decimal_point(t: ForcedNumberCategory) -> ForcedNumberCategory {
    use ForcedDecimal::{F32, F64};
    use ForcedNumberCategory::Decimal;
    match t {
        Decimal(F32(_, _)) => Decimal(F32(true, true)),
        Decimal(F64(_, _)) => Decimal(F64(true, true)),
        t => t
    }
}

fn disable_negative_sign(t: ForcedNumberCategory) -> ForcedNumberCategory {
    use ForcedDecimal::{F32, F64};
    use ForcedSigned::{I8, I16, I32, I64, I128};
    use ForcedNumberCategory::{Decimal, Signed, Unsigned};
    match t {
        Decimal(d) => match d {
            F32(_, _) => Decimal(F32(true, false)),
            F64(_, _) => Decimal(F64(true, false))
        }
        Signed(s) => match s {
            I8(_) => Signed(I8(true)),
            I16(_) => Signed(I16(true)),
            I32(_) => Signed(I32(true)),
            I64(_) => Signed(I64(true)),
            I128(_) => Signed(I128(true)),
        }
        Unsigned(u) => Unsigned(u)
    }
}

fn get_number_from_string(t: NumberType, value: &str) -> Result<Element, ParseNumberError> {
    use NumberType::{Signed, Unsigned, Decimal};
    match t {
        Signed => match value.parse::<i128>() {
            Ok(num128) => {
                match i8::try_from(num128) {
                    Ok(num8) => Ok(num8.into()),
                    Err(_) => match i16::try_from(num128) {
                        Ok(num16) => Ok(num16.into()),
                        Err(_) => match i32::try_from(num128) {
                            Ok(num32) => Ok(num32.into()),
                            Err(_) => match i64::try_from(num128) {
                                Ok(num64) => Ok(num64.into()),
                                Err(_) => Ok(num128.into())
                            }
                        }
                    }
                }
            }
            Err(e) => Err(e.into())
        }
        Unsigned => match value.parse::<u128>() {
            Ok(num128) => {
                match u8::try_from(num128) {
                    Ok(num8) => Ok(num8.into()),
                    Err(_) => match u16::try_from(num128) {
                        Ok(num16) => Ok(num16.into()),
                        Err(_) => match u32::try_from(num128) {
                            Ok(num32) => Ok(num32.into()),
                            Err(_) => match u64::try_from(num128) {
                                Ok(num64) => Ok(num64.into()),
                                Err(_) => Ok(num128.into())
                            }
                        }
                    }
                }
            }
            Err(e) => Err(e.into())
        }
        Decimal => match value.parse::<f64>() {
            Ok(num64) => match value.parse::<f32>() {
                Ok(num32) => Ok(num32.into()),
                Err(_) => Ok(num64.into())
            }
            Err(e) => Err(e.into())
        }
    }
}

fn get_number_from_forced(f: ForcedNumberCategory, value: &str) -> Result<Element, ParseNumberError> {
    use ForcedDecimal::{F32, F64};
    use ForcedSigned::{I8, I16, I32, I64, I128};
    use ForcedUnsigned::{U8, U16, U32, U64, U128};
    use ForcedNumberCategory::{Decimal, Signed, Unsigned};
    Ok(match f {
        Decimal(d) => match d {
            F32(_, _) => value.parse::<f32>()?.into(),
            F64(_, _) => value.parse::<f64>()?.into(),
        }
        Signed(s) => match s {
            I8(_) => value.parse::<i8>()?.into(),
            I16(_) => value.parse::<i16>()?.into(),
            I32(_) => value.parse::<i32>()?.into(),
            I64(_) => value.parse::<i64>()?.into(),
            I128(_) => value.parse::<i128>()?.into(),
        }
        Unsigned(u) => match u {
            U8 => value.parse::<u8>()?.into(),
            U16 => value.parse::<u16>()?.into(),
            U32 => value.parse::<u32>()?.into(),
            U64 => value.parse::<u64>()?.into(),
            U128 => value.parse::<u128>()?.into(),
        }
    })
}

use std::{fs, rc::Rc, cell::RefCell, collections::HashMap, iter::Peekable, str::Chars};
use crate::{Element, PmlStruct, Error};
mod get_value;

#[derive(Debug)]
pub(crate) enum WIPElement {
    Element(Element),
    IncompleteString(Vec<ISElem>),
    StringArray(Vec<(usize, Vec<ISElem>)>),
    Struct(Rc<RefCell<WIPStruct>>),
    StructArray(Vec<(usize, Rc<RefCell<WIPStruct>>)>),
}

#[derive(Debug)]
pub(crate) struct WIPStruct {
    pub(crate) finished_elements: HashMap<String, Element>,
    inc_strings: HashMap<String, Vec<ISElem>>,
    inc_string_arrays: HashMap<String, Vec<(usize, Vec<ISElem>)>>,
    inc_structs: HashMap<String, Rc<RefCell<WIPStruct>>>,
    inc_struct_arrays: HashMap<String, Vec<(usize, Rc<RefCell<WIPStruct>>)>>,
}

impl WIPStruct {
    pub fn new() -> Self {
        Self {
            finished_elements: HashMap::new(),
            inc_strings: HashMap::new(),
            inc_string_arrays: HashMap::new(),
            inc_structs: HashMap::new(),
            inc_struct_arrays: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: WIPElement) -> Result<(), Error> {
        match value {
            WIPElement::Element(elem) => match self.finished_elements.insert(key.clone(), elem) {
                None => (),
                Some(_) => return Err(Error::AlreadyExists {
                    key,
                })
            }
            WIPElement::IncompleteString(inc_str) =>  match self.inc_strings.insert(key.clone(), inc_str) {
                None => (),
                Some(_) => return Err(Error::AlreadyExists {
                    key,
                })
            },
            WIPElement::StringArray(arr) => match self.inc_string_arrays.insert(key.clone(), arr) {
                None => (),
                Some(_) => return Err(Error::AlreadyExists {
                    key,
                })
            },
            WIPElement::Struct(s) => match self.inc_structs.insert(key.clone(), s) {
                None => (),
                Some(_) => return Err(Error::AlreadyExists {
                    key,
                })
            },
            WIPElement::StructArray(arr) => match self.inc_struct_arrays.insert(key.clone(), arr) {
                None => (),
                Some(_) => return Err(Error::AlreadyExists {
                    key,
                })
            },
        }
        Ok(())
    }

    fn get_as_string_from_inc_struct(map: &HashMap<String, Rc<RefCell<WIPStruct>>>, key: &str) -> Option<String> {
            let (first, rest) = key.split_once('.')?;
            let wip_struct = map.get(first)?.borrow();
            match rest.split_once('.') {
                None => wip_struct.finished_elements.get(rest)?.try_into().ok(),
                Some(_) => Self::get_as_string_from_inc_struct(&wip_struct.inc_structs, rest)
            }
    }

    pub fn resolve_inc_strings(&mut self) -> (bool, bool) {
        let mut no_change = true;
        let mut incomplete_strings_temp: HashMap<String, Vec<ISElem>> = HashMap::new();
        for (key, inc_str) in &self.inc_strings {
            let mut accum_str = String::new();
            let mut split: Vec<ISElem> = Vec::new();
            for elem in inc_str {
                match elem {
                    ISElem::Literal(value) => accum_str.push_str(value),
                    ISElem::Variable(map, name) => {
                        if map.try_borrow().is_ok() {
                            if let Some(val) = map.borrow().finished_elements.get::<String>(name) {
                                accum_str.push_str(&val.to_string());
                            }
                            else if let Some(val) = Self::get_as_string_from_inc_struct(&map.borrow().inc_structs, name) {
                                accum_str.push_str(&val.to_string());
                            }
                            else {
                                split.push(ISElem::Literal(accum_str));
                                accum_str = String::new();
                                split.push(ISElem::Variable(map.clone(), name.clone()));
                            }
                        }
                        else if let Some(val) = self.finished_elements.get::<String>(name) {
                            accum_str.push_str(&val.to_string());
                        }
                        else if let Some(val) = Self::get_as_string_from_inc_struct(&self.inc_structs, name) {
                            accum_str.push_str(&val.to_string());
                        }
                        else {
                            split.push(ISElem::Literal(accum_str));
                            accum_str = String::new();
                            split.push(ISElem::Variable(map.clone(), name.clone()));
                        }
                    }
                }
            }
            if split.is_empty() {
                self.finished_elements.insert(key.clone(), accum_str.into());
                no_change = false;
            }
            else {
                split.push(ISElem::Literal(accum_str));
                incomplete_strings_temp.insert(key.clone(), split);
            }
        }
        self.inc_strings = incomplete_strings_temp;
        let done = self.inc_strings.is_empty();
        let (no_change2, done2) = self.resolve_inc_string_arrays();
        (no_change && no_change2, done && done2)
    }

    fn resolve_inc_string_arrays(&mut self) -> (bool, bool) {
        let mut no_change = true;
        let mut incomplete_string_arrays_temp = HashMap::new();
        for (key, arr) in &self.inc_string_arrays {
            let mut array_temp_not_done = Vec::new();
            let mut array_temp_done = Vec::new();
            for (id, inc_str) in arr {
                let mut accum_str = String::new();
                let mut split: Vec<ISElem> = Vec::new();
                for elem in inc_str {
                    match elem {
                        ISElem::Literal(value) => accum_str.push_str(value),
                        ISElem::Variable(map, name) => {
                            if map.try_borrow().is_ok() {
                                if let Some(val) = map.borrow().finished_elements.get::<String>(name) {
                                    accum_str.push_str(&val.to_string());
                                }
                                else if let Some(val) = Self::get_as_string_from_inc_struct(&map.borrow().inc_structs, name) {
                                    accum_str.push_str(&val.to_string());
                                }
                                else {
                                    split.push(ISElem::Literal(accum_str));
                                    accum_str = String::new();
                                    split.push(ISElem::Variable(map.clone(), name.clone()));
                                }
                            }
                            else if let Some(val) = self.finished_elements.get::<String>(name) {
                                accum_str.push_str(&val.to_string());
                            }
                            else if let Some(val) = Self::get_as_string_from_inc_struct(&self.inc_structs, name) {
                                accum_str.push_str(&val.to_string());
                            }
                            else {
                                split.push(ISElem::Literal(accum_str));
                                accum_str = String::new();
                                split.push(ISElem::Variable(map.clone(), name.clone()));
                            }
                        }
                    }
                }
                if split.is_empty() {
                    array_temp_done.push((*id, accum_str));
                    no_change = false;
                }
                else {
                    split.push(ISElem::Literal(accum_str));
                    array_temp_not_done.push((*id, split));
                }
            }
            if array_temp_not_done.is_empty() {
                self.finished_elements.insert(key.clone(), array_temp_done.into());
            }
            else {
                array_temp_not_done.append(&mut array_temp_done.into_iter().map(|(i,s)| (i, vec![ISElem::Literal(s)])).collect());
                incomplete_string_arrays_temp.insert(key.to_string(), array_temp_not_done);
            }
        }
        self.inc_string_arrays = incomplete_string_arrays_temp;
        let done = self.inc_string_arrays.is_empty();
        (no_change, done)
    }

    fn resolve_inc_strings_recursive(&self) -> (bool, bool) {
        let mut done = true;
        let mut no_change = true;
        for k in self.inc_structs.values() {
            let (nc, d) = k.borrow_mut().resolve_inc_strings();
            if !nc {
                no_change = false;
            }
            if !d {
                done = false;
            }
            let (nc, d) = k.borrow().resolve_inc_strings_recursive();
            if !nc {
                no_change = false;
            }
            if !d {
                done = false;
            }
        }
        (no_change, done)
    }

    fn resolve_inc_structs(&mut self) -> Result<PmlStruct, Error> {
        for (k, s) in &self.inc_structs {
            let struct_arrays = s.borrow().resolve_struct_arrays()?;
            for (k, v) in  struct_arrays {
                s.borrow_mut().finished_elements.insert(k, v);
            }
            if self.finished_elements.insert(k.clone(), Element::PmlStruct(Box::new(s.borrow_mut().resolve_inc_structs()?))).is_some() {
                return Err(Error::AlreadyExists{
                    key: k.to_string()
                });
            }
        }
        Ok(PmlStruct{
            elements: self.finished_elements.clone()
        })
    }

    pub(crate) fn resolve_struct_arrays(&self) -> Result<HashMap<String, Element>, Error> {
        let mut res = HashMap::new();
        for (key, arr) in &self.inc_struct_arrays {
            let mut temp_arr = Vec::new();
            for (id, s) in arr {
                loop {
                    let (no_change, done) = s.borrow_mut().resolve_inc_strings();
                    let (no_change2, done2) = s.borrow().resolve_inc_strings_recursive();
                    if done && done2 {
                        break;
                    }
                    if no_change && no_change2{
                        return Err(Error::IllegalDependency)
                    }
                }
                temp_arr.push((*id, s.borrow_mut().resolve_inc_structs()?));
            }
            res.insert(key.clone(), temp_arr.into());
        }
        Ok(res)
    }
}

#[derive(Debug)]
pub(crate) enum ISElem {
    Literal(String),
    Variable(Rc<RefCell<WIPStruct>>, String),
}

#[derive(Clone, Copy)]
enum KeyType {
    Quotes,
    NoQuotes
}

#[derive(Clone, Copy)]
enum TerminatorType {
    Struct,
    Array
}

pub(crate) struct ParseData<'a> {
    line: u32,
    column: u32,
    chars: Peekable<Chars<'a>>,
    nested_names: Vec<String>,
    nested_refs: Vec<Rc<RefCell<WIPStruct>>>,
    last_char: char,
}

impl<'a> ParseData<'a> {
    fn init(input: &'a str) -> Self {
        Self {
            line: 1,
            column: 0,
            chars: input.chars().peekable(),
            nested_names: Vec::new(),
            nested_refs: Vec::new(),
            last_char: '\0',
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        match c {
            Some('\n') => {
                self.line += 1;
                self.column = 0;
                self.last_char = '\n';
            }
            Some(c) => {
                self.column += 1;
                self.last_char = c;
            }
            None => ()
        }
        c
    }

    fn next_non_whitespace_peek(&mut self) -> Option<char> {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
                return Some(*c);
            }
            self.next_char();
        }
        None
    }

    fn has_next_non_whitespace(&mut self) -> bool {
        self.next_non_whitespace_peek().is_some()
    }

    fn next_non_whitespace(&mut self) -> Option<char> {
        while let Some(c) = self.next_char() {
            if !c.is_whitespace() {
                self.last_char = c;
                return Some(c);
            }
        }
        None
    }

    fn num_of_nested(&self) -> usize {
        self.nested_names.len()
    }

    fn get_nested_key(&self, n: usize) -> String {
        self.nested_names.iter().nth_back(n-1).expect("Going up too much nesting should have been caught earlier").clone()
    }

    fn get_struct_ref(&self, n: usize) -> Rc<RefCell<WIPStruct>> {
        self.nested_refs.iter().nth_back(n-1).expect("Going up too much nesting should have been caught earlier").clone()
    }

    fn get_full_struct_path(&self) -> String {
        self.nested_names.join(".")
    }

    fn add_nested_name(&mut self, key: String) {
        self.nested_names.push(key);
    }

    fn add_nested_ref(&mut self, link: Rc<RefCell<WIPStruct>>) {
        self.nested_refs.push(link);
    }

    fn drop_last_nested_name(&mut self) {
        self.nested_names.pop();
    }

    fn drop_last_nested_ref(&mut self) {
        self.nested_refs.pop();
    }

}

pub fn file(file: &str) -> Result<PmlStruct, Error> {
    let file_content = fs::read_to_string(file)?;
    parse_string(&file_content)
}

fn parse_string(input: &str) -> Result<PmlStruct, Error> {
    let mut parse_data = ParseData::init(input);
    let temp_struct = Rc::new(RefCell::new(WIPStruct::new()));
    parse_data.add_nested_ref(temp_struct.clone());

    while parse_data.has_next_non_whitespace() {
        let (key, value) = get_key_value_pair(&mut parse_data)?;
        temp_struct.borrow_mut().add(key, value)?;
    }
    loop {
        let (no_change, done) = temp_struct.borrow_mut().resolve_inc_strings();
        let (no_change2, done2) = temp_struct.borrow().resolve_inc_strings_recursive();
        if done && done2 {
            break;
        }
        if no_change && no_change2{
            return Err(Error::IllegalDependency)
        }
    }
    let struct_arrays = temp_struct.borrow().resolve_struct_arrays()?;
    for (k, v) in  struct_arrays {
        temp_struct.borrow_mut().finished_elements.insert(k, v);
    }
    let final_struct = temp_struct.borrow_mut().resolve_inc_structs();
    final_struct
}

fn illegal_char_err(c: char, pd: &ParseData) -> Error {
    Error::IllegalCharacter {
        char: c,
        line: pd.line,
        col: pd.column
    }
}

fn is_char_reserved(c: char) -> bool {
    ['=', ';', ',', '<', '>', '{', '}', '(', ')', '"', '[', ']', ':', '|', '.', '+', '$', '!', '?', '#'].into_iter().any(|r| r == c)
}

fn get_key_value_pair(parse_data: &mut ParseData) -> Result<(String, WIPElement), Error> {
    let key = match parse_data.next_non_whitespace_peek() {
        Some('"') => get_quoted_key(parse_data),
        Some(c) if is_char_reserved(c) => Err(illegal_char_err(c, parse_data)),
        Some(_) => get_unquoted_key(parse_data),
        None => unreachable!(),
    }?;
    let value = match parse_data.next_non_whitespace_peek() {
        Some('|'|'"') => {
            parse_data.add_nested_name(key.clone());
            let res = get_value::string(parse_data, TerminatorType::Struct)?.into();
            parse_data.drop_last_nested_name();
            res
        }
        Some('t' | 'f') => get_value::bool(parse_data, TerminatorType::Struct)?.into(),
        Some('<') => get_value::forced(parse_data, TerminatorType::Struct, &key)?.into(),
        Some('{') => {
            parse_data.add_nested_name(key.clone());
            let res = get_value::pml_struct(parse_data)?.into();
            parse_data.drop_last_nested_name();
            parse_data.drop_last_nested_ref();
            res
        }
        Some('[') => {
            parse_data.add_nested_name(key.clone());
            let res = get_value::array(parse_data)?;
            parse_data.drop_last_nested_name();
            res
        }
        Some('.' | '-') => get_value::number(parse_data, TerminatorType::Struct)?.into(),
        Some(c) if c.is_ascii_digit() => get_value::number(parse_data, TerminatorType::Struct)?.into(),
        Some(c) => Err(illegal_char_err(c, parse_data))?,
        None => Err(Error::UnexpectedEOF)?,
    };
    Ok((key, value))
}

fn get_quoted_key(parse_data: &mut ParseData) -> Result<String, Error> {
    parse_data.next_char();
    let mut key = String::new();
    while let Some(c) = parse_data.next_char() {
        match c {
            '"' => {
                if key.is_empty() {
                    return Err(Error::InvalidKey)
                }
                return match parse_data.next_non_whitespace() {
                    Some('=') => Ok(key),
                    Some(c) => Err(illegal_char_err(c, parse_data)),
                    None => Err(Error::UnexpectedEOF)
                }
            }
            c if is_char_reserved(c) => return Err(Error::InvalidKey),
            c => key.push(c)
        }
    }
    Err(Error::UnexpectedEOF)
}

fn get_unquoted_key(parse_data: &mut ParseData) -> Result<String, Error> {
    let mut key = String::new();
    while let Some(c) = parse_data.next_char() {
        match c {
            '=' => {
                if key.is_empty() {
                    return Err(Error::InvalidKey)
                }
                return Ok(key)
            }
            c if c.is_whitespace() => {
                return match parse_data.next_non_whitespace() {
                    Some('=') => Ok(key),
                    Some(c) => Err(illegal_char_err(c, parse_data)),
                    None => Err(Error::UnexpectedEOF)
                }
            }
            c if is_char_reserved(c) => return Err(Error::InvalidKey),
            c => key.push(c)
        }
    }
    Err(Error::UnexpectedEOF)
}

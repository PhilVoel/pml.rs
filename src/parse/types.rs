use std::{rc::Rc, cell::RefCell, collections::HashMap, iter::Peekable, str::Chars};
use crate::{PmlStruct, Element, errors::ParseError as Error};

type IncStringArray = Vec<(usize, Vec<ISElem>)>;
type IncStructArray = Vec<(usize, Rc<RefCell<WIPStruct>>)>;

#[derive(Debug)]
pub(crate) enum WIPElement {
    Element(Element),
    IncompleteString(Vec<ISElem>),
    StringArray(IncStringArray),
    Struct(Rc<RefCell<WIPStruct>>),
    StructArray(IncStructArray),
}

pub(crate) struct ParseData<'a> {
    pub line: u32,
    pub column: u32,
    chars: Peekable<Chars<'a>>,
    nested_names: Vec<String>,
    pub nested_refs: Vec<Rc<RefCell<WIPStruct>>>,
    pub last_char: char,
}

#[derive(Debug)]
pub(crate) struct WIPStruct {
    pub(crate) finished_elements: HashMap<String, Element>,
    inc_strings: HashMap<String, Vec<ISElem>>,
    inc_string_arrays: HashMap<String, IncStringArray>,
    inc_structs: HashMap<String, Rc<RefCell<WIPStruct>>>,
    inc_struct_arrays: HashMap<String, IncStructArray>,
}


#[derive(Debug)]
pub(crate) enum ISElem {
    Literal(String),
    Variable(Rc<RefCell<WIPStruct>>, String),
}

#[derive(Clone, Copy)]
pub(crate) enum KeyType {
    Quotes,
    NoQuotes
}

#[derive(Clone, Copy)]
pub(super) enum TerminatorType {
    Struct,
    Array
}

impl<'a> ParseData<'a> {
    pub fn init(input: &'a str) -> Self {
        Self {
            line: 1,
            column: 0,
            chars: input.chars().peekable(),
            nested_names: Vec::new(),
            nested_refs: Vec::new(),
            last_char: '\0',
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
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

    pub fn next_non_whitespace_peek(&mut self) -> Option<char> {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
                return Some(*c);
            }
            self.next_char();
        }
        None
    }

    pub fn has_next_non_whitespace(&mut self) -> bool {
        self.next_non_whitespace_peek().is_some()
    }

    pub fn next_non_whitespace(&mut self) -> Option<char> {
        while let Some(c) = self.next_char() {
            if !c.is_whitespace() {
                self.last_char = c;
                return Some(c);
            }
        }
        None
    }

    pub fn num_of_nested(&self) -> usize {
        self.nested_names.len()
    }

    pub fn get_nested_key(&self, n: usize) -> String {
        self.nested_names.iter().nth_back(n-1).expect("Going up too much nesting should have been caught earlier").clone()
    }

    pub fn get_struct_ref(&self, n: usize) -> Rc<RefCell<WIPStruct>> {
        self.nested_refs.iter().nth_back(n-1).expect("Going up too much nesting should have been caught earlier").clone()
    }

    pub fn get_full_struct_path(&self) -> String {
        self.nested_names.join(".")
    }

    pub fn add_nested_name(&mut self, key: String) {
        self.nested_names.push(key);
    }

    pub fn add_nested_ref(&mut self, link: Rc<RefCell<WIPStruct>>) {
        self.nested_refs.push(link);
    }

    pub fn drop_last_nested_name(&mut self) {
        self.nested_names.pop();
    }

    pub fn drop_last_nested_ref(&mut self) {
        self.nested_refs.pop();
    }

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

    pub fn resolve_inc_strings_recursive(&self) -> (bool, bool) {
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

    pub fn resolve_inc_structs(&mut self) -> Result<PmlStruct, Error> {
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

use std::collections::{HashMap, VecDeque};

use super::{
    combinator::{parse_ref, Matcher},
    peg_matcher::PegMatcher,
    peg_rules::init_peg_parser,
};

pub struct PegParser<T: Clone + ParserData + 'static> {
    peg_parser: Parser<PegMatcher<T>>,
}

impl<T: Clone + ParserData + 'static> PegParser<T> {
    pub fn new() -> Self {
        PegParser {
            peg_parser: init_peg_parser::<PegMatcher<T>>(),
        }
    }
    pub fn parse_parser(&mut self, parser_rules: String) -> Result<Parser<T>, &str> {
        println!(
            "Parser File Remaining: {:?}",
            &parser_rules[self.peg_parser.pos..self.peg_parser.source_code_size]
        );
        let rules = match self.peg_parser.parse(parser_rules.as_str()) {
            Ok(rules) => match rules {
                PegMatcher::Rules(a) => a,
                _ => {
                    panic!("Parse failed.");
                }
            },
            Err(str) => {
                return Err(str);
            }
        };
        let mut output_parser = Parser::new();
        for rule in rules {
            output_parser.add_rule(rule.0, rule.1);
        }
        Ok(output_parser)
    }
}

pub trait ParserData: Sized + Clone {
    fn string(pos: (usize, usize), str: String) -> Self;
    fn keywords(pos: (usize, usize), parser: &mut Parser<Self>) -> Self;
    fn null() -> Self;
    fn data(pos: (usize, usize), name: &str, parser: &mut Parser<Self>) -> Self;
    fn is_null(&self) -> bool;
}

#[derive(Clone)]
pub struct Parser<T: Clone + ParserData> {
    pub grammar_list: HashMap<String, Matcher<T>>,
    // pub data: HashMap<String, T>,
    pub data: Vec<HashMap<String, T>>,
    pub pos: usize,
    pub error_pos: usize,
    pub source_code_size: usize,
}

impl<T: Clone + ParserData + 'static> Parser<T> {
    pub fn new() -> Parser<T> {
        Parser {
            grammar_list: HashMap::new(),
            // data: HashMap::new(),
            data: Vec::new(),
            pos: 0,
            error_pos: 0,
            source_code_size: 0,
        }
    }
    pub fn reset(&mut self) {
        self.data = Vec::new();
        self.error_pos = 0;
        self.pos = 0;
    }
    pub fn add_rule(&mut self, name: String, rule: Matcher<T>) {
        self.grammar_list.insert(name, rule);
    }
    pub fn enter_scope(&mut self) {
        self.data.push(HashMap::new());
    }
    pub fn exit_scope(&mut self) {
        self.data.pop();
    }
    pub fn add_data(&mut self, name: String, data: T) {
        if !data.is_null() {
            // println!("Adding data {}", name);
            // self.data
            //     .insert(self.scopes.last().unwrap().clone() + ":" + &name, data);
            let len = self.data.len();
            // println!("{} in {}", name, len);
            if len >= 1 {
                self.data[len - 1].insert(name, data);
            } else {
                panic!("Parser Stack does not exist.");
            }
        }
    }
    // filter is_null
    pub fn get_data(&mut self, name: &str) -> Option<T> {
        // println!("{}", size_of::<HashMap<&str, T>>());
        // println!(
        //     "{:?} in {}",
        //     self.data.last().unwrap().keys(),
        //     self.data.len()
        // );
        // println!("Getting data: {:?}", name);
        match self.data.last_mut() {
            Some(map) => {
                return match map.get_mut(&name.to_string()) {
                    Some(data) => {
                        let mut temp = T::null();
                        std::mem::swap(data, &mut temp);
                        Some(temp)
                    }
                    None => None,
                };
            }
            None => {
                panic!("Parser stack does not exist.");
            }
        }
    }
    pub fn get_data_from_parent_scope(&mut self, name: &str) -> Option<T> {
        // println!("{}", size_of::<HashMap<&str, T>>());
        // println!("{:?}", self.data.keys());
        let len = self.data.len();
        let val = if len >= 2 {
            match self
                .data
                .get_mut(len - 2)
                .expect("Stack does not exist.")
                .get_mut(&name.to_string())
            {
                Some(data) => {
                    let mut temp = T::null();
                    std::mem::swap(data, &mut temp);
                    Some(temp)
                }
                None => None,
            }
        } else {
            None
        };
        val
    }
    pub fn eat(&mut self, str: &str) -> bool {
        // println!("eaten {}", str);
        if self.pos + str.chars().count() <= self.source_code_size {
            self.pos += str.chars().count();
            true
        } else {
            println!("could not eat: {}", str);
            false
        }
        // println!("Remaining::: {}", self.source_code);
    }
    pub fn backtrace(
        &mut self,
        pos: usize,
        data_pos: &(usize, Vec<String>),
    ) -> (bool, VecDeque<HashMap<String, T>>) {
        let mut erase = VecDeque::new();
        // println!("{} and {:?}", data_pos.0, data_pos.1);
        while self.data.len() > data_pos.0 {
            erase.push_front(self.data.pop().expect("Stack does not exist."));
        }
        erase.push_front(HashMap::new());
        if self.data.len() == 0 {
            panic!("Stack does not exist.");
        } else if self.data.len() == data_pos.0 {
            let len = self.data.len();
            // println!("now: {} and {:?}", len, self.data[len - 1].keys());
            let mut keys = vec![];
            for (k, _) in self.data[len - 1].iter() {
                if !data_pos.1.contains(k) {
                    keys.push(k.clone());
                }
            }
            for key in keys {
                erase[0].insert(
                    key.clone(),
                    self.data[len - 1]
                        .remove(&key)
                        .expect(format!("Could not find {}", key).as_str()),
                );
            }
            // println!("backtrace erase {} {}", self.pos, self.error_pos);
            if self.pos >= self.error_pos {
                self.error_pos = self.pos;
                self.pos = pos;
                (true, erase)
            } else {
                self.pos = pos;
                (false, VecDeque::new())
            }
        } else {
            self.pos = pos;
            panic!("Not popped enough.");
        }
    }
    pub fn lsp_backtrace(
        &mut self,
        data_pos: &(usize, Vec<String>),
    ) -> VecDeque<HashMap<String, T>> {
        let mut erase = VecDeque::new();
        // println!("{} and {:?}", data_pos.0, data_pos.1);
        while self.data.len() > data_pos.0 {
            erase.push_front(self.data.pop().expect("Stack does not exist."));
        }
        erase.push_front(HashMap::new());
        if self.data.len() == 0 {
            panic!("Stack does not exist.");
        } else if self.data.len() == data_pos.0 {
            let len = self.data.len();
            // println!("now: {} and {:?}", len, self.data[len - 1].keys());
            let mut keys = vec![];
            for (k, _) in self.data[len - 1].iter() {
                if !data_pos.1.contains(k) {
                    keys.push(k.clone());
                }
            }
            for key in keys {
                erase[0].insert(
                    key.clone(),
                    self.data[len - 1]
                        .remove(&key)
                        .expect(format!("Could not find {}", key).as_str()),
                );
            }
            erase
        } else {
            panic!("Not popped enough.");
        }
    }
    pub fn parse(&mut self, string: &str) -> Result<T, &str> {
        let source_code: Vec<char> = string.chars().collect();
        self.source_code_size = source_code.len();
        self.data.push(HashMap::new());
        match parse_ref("Start".to_string(), None)(&source_code[..], self) {
            Err(()) => {
                println!(
                    "Could not parse at pos {}\n Remaining: {:?}",
                    self.error_pos,
                    &source_code[self.error_pos..]
                );
                return Err("Start Parse failed.");
            }
            _ => {}
        }
        if self.data.len() == 1 {
            match self.data[0].get("Start") {
                Some(data) => {
                    return Ok(data.clone());
                }
                None => {
                    return Err("Parse failed: Could not get Start item.");
                }
            }
        } else {
            return Err("Parse failed: Stack does not exist.");
        }
    }
    pub fn to_max(&mut self, max_pos: usize, data: VecDeque<HashMap<String, T>>) {
        // println!("to_max");
        // for map in &data {
        //     println!("{:?}", map.keys())
        // }
        self.pos = max_pos;
        if data.len() > 0 {
            for entry in &data[0] {
                self.data
                    .last_mut()
                    .unwrap()
                    .insert(entry.0.clone(), entry.1.clone());
            }
            for map in data.iter().skip(1) {
                self.data.push(map.clone());
            }
        }
        // println!("now max");
        // for map in &self.data {
        //     println!("{:?}", map.keys())
        // }
    }
}

use std::{
    collections::VecDeque,
    io::{stderr, Write},
    sync::Arc,
};

use regex::{self, Regex};

use crate::lsp_parser::peg_parser::parser::{Parser, ParserData};

pub type LSPMatcher<T> = Arc<dyn Fn(&[char], &mut Parser<T>) -> Result<bool, ()> + Sync + Send>;

pub fn lsp_parse_str<T: ParserData + Clone + 'static>(str: String) -> LSPMatcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_str {:?}", str);
            let chars: Vec<char> = str.chars().collect();
            if str
                .chars()
                .all(|x| !char::is_ascii_punctuation(&x) && !char::is_ascii_whitespace(&x))
            {
                let keywords = T::keywords((parser.pos, parser.pos + chars.len()), parser);
                parser.add_data("keywords".to_string(), keywords);
            }
            if input.starts_with(&chars[..]) {
                if parser.eat(&str) {
                    Ok(true)
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        },
    );
}

pub fn lsp_parse_any<T: ParserData + Clone + 'static>() -> LSPMatcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_any {}", parser.input);
            if input.len() > 0 {
                let ch = input[0];
                if ch == '\n' {
                    return Err(());
                }
                if parser.eat(&ch.to_string()) {
                    Ok(true)
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        },
    );
}

pub fn lsp_parse_range<T: ParserData + Clone + 'static>(range: String) -> LSPMatcher<T> {
    let mut range_str = String::new();
    range_str += "[";
    range_str += range.as_str();
    range_str += "]";
    let range_regex = Regex::new(range_str.as_str()).expect("Range Regex Parsing Failed.");
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            if input.len() > 0 {
                if let Some(ch) = range_regex.captures(input[0].to_string().as_str()) {
                    if parser.eat(&ch[0].to_string()) {
                        Ok(true)
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        },
    );
}

pub fn lsp_parse_many<T: ParserData + Clone + 'static>(matcher: LSPMatcher<T>) -> LSPMatcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_many");
            let pos = parser.pos;
            let mut final_b = true;
            while let Ok(b) = matcher(&input[(parser.pos - pos)..], parser) {
                if !b {
                    final_b = false;
                }
            }
            Ok(final_b)
        },
    );
}

pub fn lsp_parse_more_than_one<T: ParserData + Clone + 'static>(
    matcher: LSPMatcher<T>,
) -> LSPMatcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_more_than_one");
            let pos = parser.pos;
            if let Ok(first_b) = matcher(input, parser) {
                let str = &input[(parser.pos - pos)..];
                if let Ok(b) = lsp_parse_many(matcher.clone())(str, parser) {
                    Ok(first_b || b)
                } else {
                    Ok(first_b)
                }
            } else {
                Err(())
            }
        },
    );
}

pub fn lsp_parse_not<T: ParserData + Clone + 'static>(matcher: LSPMatcher<T>) -> LSPMatcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_not");
            let pos = parser.pos;
            if let Ok(_) = matcher(input, parser) {
                parser.pos = pos;
                Err(())
            } else {
                Ok(true)
            }
        },
    );
}

pub fn lsp_parse_seq<T: ParserData + Clone + 'static>(
    matchers: Vec<LSPMatcher<T>>,
) -> LSPMatcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_seq");
            let pos = parser.pos;
            let mut keys = vec![];
            for key in parser.data.last().expect("Stack does not exist.").keys() {
                keys.push(key.clone());
            }
            let data_pos = (parser.data.len(), keys);
            for matcher in &matchers {
                match matcher(&input[(parser.pos - pos)..], parser) {
                    Ok(b) => {
                        if !b {
                            return Ok(false);
                        }
                    }
                    Err(()) => {
                        if parser.pos > pos {
                            return Ok(false);
                        }
                        parser.lsp_backtrace(&data_pos);
                        parser.pos = pos;
                        return Err(());
                    }
                }
            }
            Ok(true)
        },
    );
}

pub fn lsp_parse_or<T: ParserData + Clone + 'static>(
    matchers: Vec<LSPMatcher<T>>,
) -> LSPMatcher<T> {
    // backtrack needed
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            let pos = parser.pos;
            let mut max_pos = parser.pos;
            let mut max = VecDeque::new();
            let mut keys = vec![];
            for key in parser.data.last().expect("Stack does not exist.").keys() {
                keys.push(key.clone());
            }
            let data_pos = (parser.data.len(), keys);
            // println!("parse_or");
            for matcher in &matchers {
                // println!("parse_or: {}", parser.input);
                // println!("{} in {}", parser.pos - pos, input.len());
                match matcher(input, parser) {
                    Ok(true) => {
                        return Ok(true);
                    }
                    Ok(false) | Err(()) => {
                        // println!("{:?}", &input[parser.pos - pos..]);
                        let m = parser.lsp_backtrace(&data_pos);
                        if parser.pos >= max_pos {
                            // println!("update max {} {}", pos, parser.pos);
                            max_pos = parser.pos;
                            max = m;
                        }
                        parser.pos = pos;
                        // println!("backtraced to {:?}", &input[parser.pos - pos..]);
                    }
                }
            }
            parser.to_max(max_pos, max);
            if parser.pos == pos {
                Err(())
            } else {
                Ok(false)
            }
        },
    );
}

pub fn lsp_parse_ref<T: ParserData + Clone + 'static>(
    name: String,
    save_name: Option<String>,
) -> LSPMatcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_ref {}", name);
            let matcher: LSPMatcher<T>;
            if let Some(m) = parser.grammar_list.get(name.as_str()) {
                matcher = m.clone();
            } else {
                let _ = writeln!(stderr(), "Could not find {} in the grammar.", name);
                matcher = lsp_parse_not(lsp_parse_any());
                // panic!("Could not find {} in the grammar.", name);
            }
            parser.enter_scope();
            let pos = parser.pos;
            match matcher(input, parser) {
                Ok(b) => {
                    let data = T::data((pos, parser.pos), name.as_str(), parser);
                    parser.exit_scope();
                    match save_name.clone() {
                        Some(str) => parser.add_data(str, data),
                        None => parser.add_data(name.clone(), data),
                    }
                    Ok(b)
                }
                Err(()) => {
                    // println!("Could not parse {}", name);
                    parser.exit_scope();
                    Err(())
                }
            }
        },
    );
}

pub fn lsp_capture_string<T: ParserData + Clone + 'static>(
    name: String,
    matcher: LSPMatcher<T>,
) -> LSPMatcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            let pos = parser.pos;
            match matcher(input, parser) {
                Ok(b) => {
                    parser.add_data(
                        name.clone(),
                        T::string(
                            (pos, parser.pos),
                            (&input[0..parser.pos - pos]).iter().collect(),
                        ),
                    );
                    Ok(b)
                }
                Err(()) => Err(()),
            }
        },
    );
}

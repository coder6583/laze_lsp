use std::sync::Arc;

use regex::{self, Regex};

use super::{parser::Parser, parser::ParserData};

pub type Matcher<T> = Arc<dyn Fn(&[char], &mut Parser<T>) -> Result<bool, ()> + Sync + Send>;

pub fn parse_str<T: ParserData + Clone + 'static>(str: String) -> Matcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_str {:?}", str);
            let chars: Vec<char> = str.chars().collect();
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

pub fn parse_any<T: ParserData + Clone + 'static>() -> Matcher<T> {
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

pub fn parse_range<T: ParserData + Clone + 'static>(range: String) -> Matcher<T> {
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

pub fn parse_many<T: ParserData + Clone + 'static>(matcher: Matcher<T>) -> Matcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_many");
            let pos = parser.pos;
            while let Ok(true) = matcher(&input[(parser.pos - pos)..], parser) {}
            Ok(true)
        },
    );
}

pub fn parse_more_than_one<T: ParserData + Clone + 'static>(matcher: Matcher<T>) -> Matcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_more_than_one");
            let pos = parser.pos;
            if let Ok(true) = matcher(input, parser) {
                parse_many(matcher.clone())(&input[(parser.pos - pos)..], parser)
            } else {
                Err(())
            }
        },
    );
}

pub fn parse_not<T: ParserData + Clone + 'static>(matcher: Matcher<T>) -> Matcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_not");
            let pos = parser.pos;
            if let Ok(true) = matcher(input, parser) {
                parser.pos = pos;
                Err(())
            } else {
                Ok(true)
            }
        },
    );
}

pub fn parse_seq<T: ParserData + Clone + 'static>(matchers: Vec<Matcher<T>>) -> Matcher<T> {
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
                    Ok(true) => {}
                    Err(()) | Ok(false) => {
                        parser.backtrace(pos, &data_pos);
                        return Err(());
                    }
                }
            }
            Ok(true)
        },
    );
}

pub fn parse_or<T: ParserData + Clone + 'static>(matchers: Vec<Matcher<T>>) -> Matcher<T> {
    // backtrack needed
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            let pos = parser.pos;
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
                    Err(()) | Ok(false) => {
                        parser.backtrace(pos, &data_pos);
                    }
                }
            }
            Err(())
        },
    );
}

pub fn parse_ref<T: ParserData + Clone + 'static>(
    name: String,
    save_name: Option<String>,
) -> Matcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            // println!("parse_ref {}", name);
            let matcher: Matcher<T>;
            if let Some(m) = parser.grammar_list.get(name.as_str()) {
                matcher = m.clone();
            } else {
                panic!("Could not find {} in the grammar.", name);
            }
            parser.enter_scope();
            let pos = parser.pos;
            match matcher(input, parser) {
                Ok(true) => {
                    let data = T::data((pos, parser.pos), name.as_str(), parser);
                    // println!("parsed: {name}");
                    parser.exit_scope();
                    match save_name.clone() {
                        Some(str) => parser.add_data(str, data),
                        None => parser.add_data(name.clone(), data),
                    }
                    Ok(true)
                }
                Err(()) | Ok(false) => {
                    parser.exit_scope();
                    Err(())
                }
            }
        },
    );
}

pub fn capture_string<T: ParserData + Clone + 'static>(
    name: String,
    matcher: Matcher<T>,
) -> Matcher<T> {
    return Arc::new(
        move |input: &[char], parser: &mut Parser<T>| -> Result<bool, ()> {
            let pos = parser.pos;
            match matcher(input, parser) {
                Ok(true) => {
                    parser.add_data(
                        name.clone(),
                        T::string(
                            (pos, parser.pos),
                            (&input[0..parser.pos - pos]).iter().collect(),
                        ),
                    );
                    Ok(true)
                }
                Err(()) | Ok(false) => Err(()),
            }
        },
    );
}

#[allow(unused_imports)]
use std::io::{stderr, Write};

#[allow(unused_imports)]
use super::combinator::parse_or;
#[allow(unused_imports)]
use super::parser::ParserData;
#[allow(unused_imports)]
use super::peg_matcher::PegMatcher;
#[allow(unused_imports)]
use super::{parser::Parser, peg_rules::init_peg_parser};
#[allow(dead_code)]
const PEG: &str = r#"GreetWord = {"Hi" / "Hello" / "Goodbye": word}
ID = {[㐀-龯ぁ-んァ-ヶa-zA-Z_ー]+: id}
Greeting = GreetWord ID "!" / GreetWord ID " "+ "and" ID "!"
Greetings = Greeting ( Greeting )*
Start = Greetings"#;

// need to make capture string function

#[allow(dead_code)]
const GREETING: &str = "Hi 成田 and 成田!
Hello 永田!
Goodbye 永田!";

#[test]
fn test_peg_parser() {
    #[derive(Clone, Debug, PartialEq)]
    enum GreetingData {
        StringData(String),
        Greeting((String, String)),
        Greetings(Vec<(String, String)>),
        None,
    }
    impl GreetingData {
        fn get_string_data(&self) -> String {
            if let Self::StringData(str) = self {
                str.clone()
            } else {
                "".to_string()
            }
        }
    }
    impl ParserData for GreetingData {
        fn string(_pos: (usize, usize), str: String) -> Self {
            Self::StringData(str)
        }
        fn null() -> Self {
            Self::None
        }
        fn keywords(_: (usize, usize), _: &mut Parser<Self>) -> Self {
            Self::None
        }
        fn data(_pos: (usize, usize), name: &str, parser: &mut Parser<GreetingData>) -> Self {
            fn extract_string_data(data: Option<GreetingData>, name: &str, rule: &str) -> String {
                match data {
                    Some(data) => data.get_string_data(),
                    None => {
                        let _ = writeln!(
                            stderr(),
                            "Could not find \"{}\" in the grammar to reduce \"{}\"",
                            name,
                            rule
                        );
                        "".to_string()
                    }
                }
            }
            println!("Reducing: {}", name);
            match name {
                "GreetWord" => Self::StringData(extract_string_data(
                    parser.get_data("word"),
                    "word",
                    "GreetWord",
                )),
                "ID" => Self::StringData(extract_string_data(parser.get_data("id"), "id", "ID")),
                "Greeting" => match parser.get_data_from_parent_scope("Greeting") {
                    Some(data) => match data {
                        Self::Greeting(greeting) => Self::Greetings(vec![
                            greeting,
                            (
                                extract_string_data(parser.get_data("ID"), "ID", "Greeting"),
                                extract_string_data(
                                    parser.get_data("GreetWord"),
                                    "GreetWord",
                                    "Greeting",
                                ),
                            ),
                        ]),
                        Self::Greetings(mut greetings) => {
                            greetings.push((
                                extract_string_data(parser.get_data("ID"), "ID", "Greeting"),
                                extract_string_data(
                                    parser.get_data("GreetWord"),
                                    "GreetWord",
                                    "Greeting",
                                ),
                            ));
                            Self::Greetings(greetings)
                        }
                        _ => {
                            let _ = writeln!(
                                stderr(),
                                "Greeting does not have type Greeting or Greetings."
                            );
                            Self::None
                        }
                    },
                    None => Self::Greeting((
                        extract_string_data(parser.get_data("ID"), "ID", "Greeting"),
                        extract_string_data(parser.get_data("GreetWord"), "GreetWord", "Greeting"),
                    )),
                },
                "Greetings" => match parser.get_data("Greeting") {
                    Some(greeting) => match greeting {
                        Self::Greeting(data) => Self::Greetings(vec![data]),
                        Self::Greetings(data) => Self::Greetings(data),
                        _ => Self::Greetings(vec![]),
                    },
                    None => Self::Greetings(vec![]),
                },
                "Start" => parser.get_data("Greetings").expect("Start"),
                str => {
                    let _ = writeln!(stderr(), "What is this token: {}", str);
                    Self::None
                }
            }
        }
        fn is_null(&self) -> bool {
            if let Self::None = self {
                true
            } else {
                false
            }
        }
    }
    let mut peg_parser = init_peg_parser::<PegMatcher<GreetingData>>();
    let rules = match peg_parser.parse(PEG).expect("Parsing rules") {
        PegMatcher::Rules(a) => a,
        _ => {
            panic!("Parse failed.");
        }
    };
    println!("Parsed rules");
    let mut test_parser: Parser<GreetingData> = Parser::new();
    for rule in rules {
        test_parser.add_rule(rule.0.clone(), rule.1.clone());
    }
    match test_parser.parse(GREETING) {
        Ok(greetings) => {
            assert_eq!(test_parser.pos, GREETING.chars().count());
            match greetings {
                GreetingData::Greetings(data) => {
                    for s in &data {
                        println!("Name: {}, Greeting: {}", s.0, s.1);
                    }
                }
                _ => {
                    panic!("Greetings is not of right type -> Parse Failed.")
                }
            }
        }
        Err(_) => {
            panic!("Parse failed at position {}.", test_parser.pos);
        }
    }
}

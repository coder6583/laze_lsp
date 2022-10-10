use std::io::{stderr, Write};

use crate::lsp_parser::lsp_token::lsp_combinator::{LSPMatcher, *};

use super::{parser::Parser, parser::ParserData};

use super::extracter::*;

#[derive(Clone)]
pub enum PegMatcher<T: ParserData + Clone + 'static> {
    Rules(Vec<(String, LSPMatcher<T>)>),
    #[allow(dead_code)]
    Rule((String, LSPMatcher<T>)),
    Matcher(LSPMatcher<T>),
    Matchers(Vec<LSPMatcher<T>>),
    String(String),
    None,
}

impl<T: ParserData + Clone> PegMatcher<T> {
    pub fn get_string_data(self, name: &str, rule: &str) -> String {
        if let Self::String(str) = self {
            str
        } else {
            let _ = writeln!(stderr(), "{} in {} is not a string.", name, rule);
            "".to_string()
        }
    }
    pub fn get_matcher_data(self, name: &str, rule: &str) -> LSPMatcher<T> {
        if let Self::Matcher(matcher) = self {
            matcher
        } else {
            panic!("{} in {} is not a matcher.", name, rule);
        }
    }
    pub fn get_matchers_data(self, name: &str, rule: &str) -> Vec<LSPMatcher<T>> {
        if let Self::Matchers(matcher) = self {
            matcher
        } else {
            panic!("{} in {} is not a matcher.", name, rule);
        }
    }
    pub fn get_rules_data(self, name: &str, rule: &str) -> Vec<(String, LSPMatcher<T>)> {
        if let Self::Rules(matcher) = self {
            matcher
        } else {
            panic!("{} in {} is not a matcher.", name, rule);
        }
    }
}

impl<T: ParserData + Clone + 'static> ParserData for PegMatcher<T> {
    fn string(_: (usize, usize), str: String) -> Self {
        Self::String(str)
    }
    fn null() -> Self {
        Self::None
    }
    fn keywords(_: (usize, usize), _: &mut Parser<Self>) -> Self {
        Self::None
    }
    fn data(_: (usize, usize), name: &str, parser: &mut Parser<Self>) -> Self {
        // println!("Reducing: {}", name);
        match name {
            "StringContent" => {
                let content = extract_string_data(parser.get_data("content"), "content", name);
                let newcontent = match content.as_str() {
                    "\\\"" => "\"".to_string(),
                    "\\\\" => "\\".to_string(),
                    "\\n" => "\n".to_string(),
                    str => str.to_string(),
                };
                // println!("StringContent: {newcontent}");
                Self::String(newcontent)
            }
            "String" => Self::Matcher(lsp_parse_seq(vec![
                lsp_parse_str(
                    extract_string_data(parser.get_data("StringContent"), "StringContent", name)
                        .clone(),
                ),
                lsp_parse_many(lsp_parse_or(vec![
                    lsp_parse_str(" ".to_string()),
                    lsp_parse_str("\n".to_string()),
                    lsp_parse_str("\t".to_string()),
                    lsp_parse_str("\r\n".to_string()),
                ])),
            ])),
            "RangeContent" => Self::String(extract_string_data(
                parser.get_data("content"),
                "content",
                name,
            )),
            "Range" => Self::Matcher(lsp_parse_range(extract_string_data(
                parser.get_data("RangeContent"),
                "RangeContent",
                name,
            ))),
            "NonTerminal" => {
                Self::String(extract_string_data(parser.get_data("name"), "name", name))
            }
            "NonTerminalToken" => Self::Matcher(lsp_parse_ref(
                extract_string_data(parser.get_data("NonTerminal"), "NonTerminal", name),
                match parser.get_data("Rename") {
                    Some(matcher) => match matcher {
                        Self::String(str) => Some(str),
                        _ => {
                            panic!("Rename is not matcher or null.");
                        }
                    },
                    None => None,
                },
            )),
            "Token" => match parser.get_data_from_parent_scope("Token") {
                Some(matcher) => match matcher {
                    PegMatcher::Matcher(m) => Self::Matchers(vec![
                        m,
                        extract_matcher_data(parser.get_data("tokendata"), "tokendata", name),
                    ]),
                    PegMatcher::Matchers(mut m) => {
                        m.push(extract_matcher_data(
                            parser.get_data("tokendata"),
                            "tokendata",
                            name,
                        ));
                        Self::Matchers(m)
                    }
                    _ => {
                        panic!("The last token is not a matcher.");
                    }
                },
                None => Self::Matchers(vec![extract_matcher_data(
                    parser.get_data("tokendata"),
                    "tokendata",
                    name,
                )]),
            },
            "AnyToken" => Self::Matcher(lsp_parse_any()),
            "RawToken" => Self::Matcher(extract_matcher_data(
                parser.get_data("tokendata"),
                "tokendata",
                name,
            )),
            "ManyToken" => Self::Matcher(lsp_parse_many(extract_matcher_data(
                parser.get_data("RawToken"),
                "RawToken",
                name,
            ))),
            "MoreThanOneToken" => Self::Matcher(lsp_parse_more_than_one(extract_matcher_data(
                parser.get_data("RawToken"),
                "RawToken",
                name,
            ))),
            "NotToken" => Self::Matcher(lsp_parse_not(extract_matcher_data(
                parser.get_data("RawToken"),
                "RawToken",
                name,
            ))),
            "Tokens" => match parser.get_data_from_parent_scope("Tokens") {
                Some(matcher) => match matcher {
                    PegMatcher::Matcher(m) => {
                        let mut matchers = vec![m];
                        matchers.push(lsp_parse_seq(extract_matchers_data(
                            parser.get_data("Token"),
                            "Token",
                            name,
                        )));
                        Self::Matchers(matchers)
                    }
                    PegMatcher::Matchers(mut m) => {
                        m.push(lsp_parse_seq(extract_matchers_data(
                            parser.get_data("Token"),
                            "Token",
                            name,
                        )));
                        Self::Matchers(m)
                    }
                    _ => {
                        panic!("The last token is not a matcher.");
                    }
                },
                None => {
                    let matchers = extract_matchers_data(parser.get_data("Token"), "Token", name);
                    Self::Matcher(lsp_parse_seq(matchers))
                }
            },
            "ParenTokens" => Self::Matcher(extract_matcher_data(
                parser.get_data("OrTokens"),
                "OrTokens",
                name,
            )),
            "OrTokens" => match parser.get_data("Tokens") {
                Some(m) => match m {
                    PegMatcher::Matcher(m) => PegMatcher::Matcher(m),
                    PegMatcher::Matchers(m) => PegMatcher::Matcher(lsp_parse_or(m)),
                    _ => {
                        panic!("Tokens is not Matcher or Matchers in OrTokens.");
                    }
                },
                None => {
                    panic!("Could not find Tokens in OrTokens.");
                }
            },
            "CaptureString" => Self::Matcher(lsp_capture_string(
                extract_string_data(parser.get_data("NonTerminal"), "NonTerminal", name),
                extract_matcher_data(parser.get_data("OrTokens"), "OrTokens", name),
            )),
            // look in parent scope
            "Rule" => {
                let rule_name =
                    extract_string_data(parser.get_data("NonTerminal"), "NonTerminal", name);
                match parser.get_data_from_parent_scope("Rule") {
                    Some(matcher) => match matcher {
                        PegMatcher::Rule(r) => Self::Rules(vec![
                            r,
                            (
                                rule_name,
                                extract_matcher_data(parser.get_data("OrTokens"), "OrTokens", name),
                            ),
                        ]),
                        PegMatcher::Rules(mut r) => {
                            r.push((
                                rule_name,
                                extract_matcher_data(parser.get_data("OrTokens"), "OrTokens", name),
                            ));
                            Self::Rules(r)
                        }
                        _ => {
                            panic!("The last token is not a matcher.");
                        }
                    },
                    None => Self::Rules(vec![(
                        rule_name,
                        extract_matcher_data(parser.get_data("OrTokens"), "OrTokens", name),
                    )]),
                }
            }
            "Rules" => Self::Rules(extract_rules_data(parser.get_data("Rule"), "Rule", name)),
            "Start" => Self::Rules(extract_rules_data(parser.get_data("Rules"), "Rules", name)),
            str => {
                let _ = writeln!(stderr(), "What is this token: {}.", str);
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

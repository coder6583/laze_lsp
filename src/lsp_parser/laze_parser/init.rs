use std::path::Path;

use crate::{
    lsp_parser::{
        ast::ast::ASTNode,
        peg_parser::parser::{Parser, PegParser},
    },
    util::file_opener::open_file,
};

pub fn init_laze_parser(parser_file_path: &Path) -> Parser<ASTNode> {
    let parser_rules = open_file(parser_file_path);
    init_laze_parser_direct(parser_rules.as_str())
}

pub fn init_laze_parser_direct(parser_rules: &str) -> Parser<ASTNode> {
    let mut laze_parser = PegParser::<ASTNode>::new();
    laze_parser
        .parse_parser(parser_rules.to_string())
        .expect("Parsing parser: ")
}

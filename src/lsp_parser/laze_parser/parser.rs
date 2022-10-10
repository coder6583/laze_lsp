use std::{
    io::{stderr, Write},
    path::Path,
};

use crate::{
    lsp_parser::{ast::ast::ASTNode, peg_parser::parser::Parser},
    util::file_opener::open_file,
};

use super::init::{init_laze_parser, init_laze_parser_direct};

#[derive(Clone)]
pub struct LazeParser {
    parser: Parser<ASTNode>,
}

impl LazeParser {
    pub fn new(parser_file_path: &Path) -> Self {
        Self {
            parser: init_laze_parser(parser_file_path),
        }
    }
    pub fn reset(&mut self) {
        self.parser.reset();
    }
    pub fn new_direct(parser_rules: &str) -> Self {
        Self {
            parser: init_laze_parser_direct(parser_rules),
        }
    }
    pub fn parse(&mut self, program_path: &Path) -> ASTNode {
        let content = open_file(program_path);
        self.parse_direct(&content)
    }
    pub fn parse_direct(&mut self, program: &String) -> ASTNode {
        match self.parser.parse(program.as_str()) {
            Ok(node) => node,
            Err(mes) => {
                let _ = writeln!(stderr(), "{mes}");
                ASTNode::None
            }
        }
    }
}

use std::{collections::HashMap, path::Path, sync::Arc};

use tokio::sync::Mutex;

use crate::lsp_parser::laze_parser::parser::LazeParser;

#[derive(Clone)]
pub struct LSPParser {
    pub parser: LazeParser,
    pub input_str: String,
    pub lang_id: String,
    pub lines_size: Vec<usize>,
}
impl LSPParser {
    pub fn new(lang_id: String, input_str: String) -> Self {
        Self {
            parser: LazeParser::new(Path::new(
                format!("./parser_files/{}.peg", lang_id).as_str(),
            )),
            lines_size: input_str
                .chars()
                .collect::<Vec<char>>()
                .split(|c| *c == '\n')
                .map(|chars| chars.len())
                .collect(),
            input_str,
            lang_id,
        }
    }
    pub fn update_input(&mut self, input_str: String) {
        self.lines_size = input_str
            .chars()
            .collect::<Vec<char>>()
            .split(|c| *c == '\n')
            .map(|chars| chars.len())
            .collect();
        self.input_str = input_str;
        self.parser_reset();
    }
    pub fn parser_reset(&mut self) {
        self.parser.reset();
    }
}

pub type LSPParsers = Arc<Mutex<HashMap<String, LSPParser>>>;

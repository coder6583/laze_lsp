use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

use crate::{
    lsp_parser::lsp_token::{to_semantic::to_semantic_tokens, tokenize::tokenize_ast},
    util::file_opener::open_file,
    UserData,
};

use super::lsp_user::{LSPParser, LSPParsers};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InitResParam {
    r#type: i32,
    message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InitRes {
    pub jsonrpc: String,
    pub method: String,
    pub params: InitResParam,
}

pub async fn lsp_handler(json_str: &str, user_data: &mut UserData) {
    let json: Value = serde_json::from_str(json_str).expect("Parsing Request Json");
    let result = match json["method"].as_str() {
        Some(method) => match method {
            "initialize" => handle_initialize(&user_data.tx),
            "initialized" => json!(()),
            "textDocument/didOpen" => {
                handle_did_open(&json["params"], &mut user_data.parsers).await
            }
            "textDocument/didChange" => {
                handle_did_change(&json["params"], &mut user_data.parsers).await
            }
            "textDocument/semanticTokens/full" => {
                handle_semantic_full(&json["params"], &mut user_data.parsers).await
            }
            _ => {
                println!("Could not access method: {}", method);
                json!(())
            }
        },
        None => {
            eprintln!("Could not find method in request json.");
            json!(())
        }
    };
    if let Some(id) = json["id"].as_i64() {
        let new_response = json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        });
        user_data
            .tx
            .send(Message::text(new_response.to_string()))
            .expect("User does not exist.");
    }
}

pub fn handle_initialize(tx: &UnboundedSender<Message>) -> Value {
    let response = json!({
        "jsonrpc": "2.0",
        "method": "window/logMessage",
        "params": {
            "type": 4,
            "message": "Server Initialized."
        }
    });
    tx.send(Message::text(response.to_string()))
        .expect("The User does not exist anymore.");
    let settings_str = open_file(Path::new("./src/server/settings.json"));
    let settings: Value =
        serde_json::from_str(settings_str.as_str()).expect("Parsing LSP Settings.");
    json!({"capabilities": settings["capabilities"]})
}

pub async fn handle_did_open(params: &Value, parsers: &mut LSPParsers) -> Value {
    let text_document = &params["textDocument"];

    parsers.lock().await.insert(
        text_document["uri"].as_str().unwrap().to_string(),
        LSPParser::new(
            text_document["languageId"].as_str().unwrap().to_string(),
            text_document["text"].as_str().unwrap().to_string(),
        ),
    );
    json!(())
}

pub async fn handle_did_change(params: &Value, parsers: &mut LSPParsers) -> Value {
    let text_document = &params["textDocument"];

    let new_content = params["contentChanges"]
        .as_array()
        .expect("Content change is not an array in params")
        .get(0)
        .expect("Element 0 doesn't exist in contentChanges.")["text"]
        .as_str()
        .expect("Could not get text from json request.")
        .to_string();

    parsers
        .lock()
        .await
        .get_mut(&text_document["uri"].as_str().unwrap().to_string())
        .unwrap()
        .update_input(new_content);

    json!(())
}

pub async fn handle_semantic_full(params: &Value, parsers: &mut LSPParsers) -> Value {
    let uri = params["textDocument"]["uri"].as_str().unwrap().to_string();

    if let Some(parser_info) = parsers.lock().await.get_mut(&uri) {
        parser_info.parser_reset();
        let ast = parser_info.parser.parse_direct(&parser_info.input_str);
        let tokenlist = tokenize_ast(ast);
        json!({"data": to_semantic_tokens(tokenlist, &parser_info.lines_size)})
    } else {
        json!({"data": []})
    }
}

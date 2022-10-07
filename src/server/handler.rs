use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::mpsc::UnboundedSender;
use warp::{ws::Message, Reply};

use crate::util::file_opener::open_file;

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

pub fn lsp_handler(json_str: &str, tx: &UnboundedSender<Message>) {
    let json: Value = serde_json::from_str(json_str).expect("Parsing Request Json");
    match json["method"].as_str() {
        Some(method) => match method {
            "initialize" => {
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
                let new_response = json!({
                    "jsonrpc": "2.0",
                    "id": 0,
                    "result": {"capabilities": settings["capabilities"]}
                });
                tx.send(Message::text(new_response.to_string()))
                    .expect("The User does not exist anymore.");
            }
            _ => {
                println!("Could not access method: {}", method);
            }
        },
        None => {
            eprintln!("Could not find method in request json.");
        }
    }
}

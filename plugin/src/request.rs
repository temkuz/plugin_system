use std::process;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Response;

fn default_jsonrpc() -> String {
    "2.0".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(default = "default_jsonrpc")]
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: i32,
}

impl Request {
    /// Method for all plugins for read from stdin
    pub fn read_stdin() -> Self {
        let r = serde_json::from_reader(std::io::stdin());
        match r {
            Ok(r) => r,
            Err(e) => {
                Response::invalid_json(&e.to_string()).write();
                process::exit(1)
            }
        }
    }
}

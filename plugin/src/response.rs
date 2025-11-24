use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessResponse {
    pub jsonrpc: String,
    pub result: Value,
    pub id: i32,
}

impl SuccessResponse {
    pub fn new(result: Value, id: i32) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result,
            id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FailedResponseError {
    pub code: Option<i32>,
    pub error: Value,
}

impl FailedResponseError {
    pub fn new(code: Option<i32>, error: Value) -> Self {
        Self { code, error }
    }

    pub fn invalid_json(error: &str) -> Self {
        let code = Some(-32700);
        let error = format!("invalid_json: '{error}'");
        Self::new(code, serde_json::to_value(error).unwrap())
    }

    pub fn method_not_found(method: &str) -> Self {
        let code = Some(-32601);
        let error = format!("'{method}' not found");
        Self::new(code, serde_json::to_value(error).unwrap())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FailedResponse {
    pub jsonrpc: String,
    pub error: FailedResponseError,
    pub id: Option<i32>,
}

impl FailedResponse {
    pub fn new(error: FailedResponseError, id: Option<i32>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            error,
            id,
        }
    }

    pub fn invalid_json(error: &str) -> Self {
        Self::new(FailedResponseError::invalid_json(error), None)
    }

    pub fn method_not_found(method: &str, id: i32) -> Self {
        Self::new(FailedResponseError::method_not_found(method), Some(id))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Response {
    Success(SuccessResponse),
    Failed(FailedResponse),
}

impl Response {
    pub fn success(value: Value, id: i32) -> Self {
        Self::Success(SuccessResponse::new(value, id))
    }

    pub fn failed(value: FailedResponseError, id: Option<i32>) -> Self {
        Self::Failed(FailedResponse::new(value, id))
    }

    pub fn write(&self) {
        let mut output = std::io::stdout();
        let _ = output.write_all(serde_json::to_string(&self).unwrap().as_bytes());
        let _ = output.flush();
    }

    pub fn invalid_json(error: &str) -> Self {
        Self::Failed(FailedResponse::invalid_json(error))
    }

    pub fn method_not_found(method: &str, id: i32) -> Self {
        Self::Failed(FailedResponse::method_not_found(method, id))
    }
}

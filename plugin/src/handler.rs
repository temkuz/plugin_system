use std::collections::HashMap;

use super::{Request, Response};

type Command = fn(Request) -> Response;
type InnerCommand = fn(&Handler, Request) -> Response;

#[derive(Default)]
pub struct Handler {
    inner_handlers: HashMap<String, fn(&Handler, Request) -> Response>,
    handlers: HashMap<String, Command>,
}

impl Handler {
    pub fn new() -> Self {
        let mut handler = Self::default();
        handler.register_inner("rpc.list_methods", Self::list_methods);
        handler
    }

    pub fn register_inner(&mut self, method: &str, f: InnerCommand) {
        self.inner_handlers.insert(method.to_string(), f);
    }

    pub fn register(&mut self, method: &str, f: Command) {
        self.handlers.insert(method.to_string(), f);
    }

    pub fn execute(&self, request: Request) -> Response {
        if let Some(command) = self.inner_handlers.get(&request.method) {
            return command(self, request);
        }

        if let Some(command) = self.handlers.get(&request.method) {
            return command(request);
        }

        Response::method_not_found(&request.method, request.id)
    }

    fn list_methods(&self, request: Request) -> Response {
        let inner_handler: Vec<&String> = self.inner_handlers.keys().collect();
        let keys: Vec<&String> = self.handlers.keys().collect();
        let result: Vec<&String> = inner_handler.into_iter().chain(keys).collect();
        Response::success(serde_json::to_value(result).unwrap(), request.id)
    }
}

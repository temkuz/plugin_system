use plugin::{Handler, Request, Response};

fn ping(request: Request) -> Response {
    let value = serde_json::to_value("pong").unwrap();
    Response::success(value, request.id)
}

fn main() {
    let request = Request::read_stdin();

    let mut handler = Handler::new();
    handler.register("ping", ping);

    handler.execute(request).write();
}

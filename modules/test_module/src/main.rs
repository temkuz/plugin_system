use plugin::{Handler, Request, Response};

fn ping(request: Request) -> Response {
    //! An example of a function that ignores input parameters and always returns success response

    let value = serde_json::to_value("pong").unwrap();
    Response::success(value, request.id)
}

fn main() {
    let request = Request::read_stdin();

    let mut handler = Handler::new();
    handler.register("ping", ping);

    handler.execute(request).write();
}

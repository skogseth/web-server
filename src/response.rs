use hyper::{Body, Response, StatusCode};

static INDEX: &[u8] = b"<a href=\"hello.html\">hello.html</a>";
static HELLO: &[u8] = b"Hello Internet!";
static NOTFOUND: &[u8] = b"Not Found";

pub fn index() -> Response<Body> {
    Response::new(INDEX.into())
}

pub fn hello() -> Response<Body> {
    Response::new(HELLO.into())
}

pub fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}
use hyper::{Body, Response, StatusCode};

use std::collections::HashMap;
use std::error::Error;
use std::result::Result;
use std::sync::Arc;

type GenericError = Box<dyn Error + Send + Sync>;
type GenericResult<T> = Result<T, GenericError>;


static INDEX: &[u8] = b"<a href=\"hello.html\">hello.html</a>";
static HELLO: &[u8] = b"Hello Internet!";
static NOTFOUND: &[u8] = b"Not Found";

pub fn index() -> GenericResult<Response<Body>> {
    Ok(Response::new(INDEX.into()))
}

pub fn hello() -> GenericResult<Response<Body>> {
    Ok(Response::new(HELLO.into()))
}

pub fn general(path: &str, files: Arc<HashMap<String, String>>) -> GenericResult<Response<Body>> {
    match files.get(path) {
        Some(file) => Ok(Response::new(file.clone().into())),
        None => not_found(),
    }
}

pub fn not_found() -> GenericResult<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap())
}
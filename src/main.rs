use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::result::Result;
use hyper::{Body, Method, Request, Response, Server, StatusCode};


type GenericError = Box<dyn Error + Send + Sync>;
type GenericResult<T> = Result<T, GenericError>;


static INDEX: &[u8] = b"<a href=\"hello.html\">hello.html</a>";
static NOTFOUND: &[u8] = b"Not Found";

async fn handle_request(req: Request<Body>) -> GenericResult<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(Response::new(INDEX.into())),
        (&Method::GET, "/hello.html") => Ok(Response::new("Hello Internet!".into())),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    const HOST: [u8; 4] = [127, 0, 0, 1];
    const PORT: u16 = 7878;
    let addr = SocketAddr::from((HOST, PORT));
    
    let service = hyper::service::make_service_fn(|_socket| async {
        Ok::<_, Infallible>(hyper::service::service_fn(move |req| handle_request(req)))
    });

    let signal = || async { 
        tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler")
    };

    let server = Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(signal());

    println!("Listening on http://{}", addr);

    server.await?;

    println!("\nServer shutdown succesful!");

    Ok(())
}
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::result::Result;
use hyper::{Body, Method, Request, Response, Server};

mod response;

type GenericError = Box<dyn Error + Send + Sync>;
type GenericResult<T> = Result<T, GenericError>;

async fn handle_request(req: Request<Body>, db: Arc<HashMap<String, String>>) -> GenericResult<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => response::index(),
        (&Method::GET, "/hello.html") => response::hello(),
        (&Method::GET, path) => response::general(path, db),
        _ => response::not_found(),
    }
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    let filenames = ["buttons.html"];
    let file_system = load_files(&filenames, "files/")?;
    let db = Arc::new(file_system);

    const HOST: [u8; 4] = [127, 0, 0, 1];
    const PORT: u16 = 7878;
    let addr = SocketAddr::from((HOST, PORT));
    
    let service = hyper::service::make_service_fn(|_socket| {
        let db_outer_clone = Arc::clone(&db);
        async {
            let service = hyper::service::service_fn(move |req| {
                let db_inner_clone = Arc::clone(&db_outer_clone);
                handle_request(req, db_inner_clone)
            });
            Ok::<_, Infallible>(service)
        }
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


use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn load_files(filenames: &[&str], root: &str) -> GenericResult<HashMap<String, String>> {
    let mut files = HashMap::new();

    for filename in filenames {
        let path = format!("{}{}", root, *filename);
        let mut f = File::open(path)?;

        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let key = format!("/{}", *filename);

        files.insert(key, buffer);
    }

    Ok(files)
}
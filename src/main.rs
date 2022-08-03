use std::error::Error;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::result::Result;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

type GenericError = Box<dyn Error + Send + Sync>;
type GenericResult<T> = Result<T, GenericError>;

struct DataBase {
    counter: Mutex<usize>,
}

impl DataBase {
    fn new() -> DataBase {
        DataBase { counter: Mutex::new(0) }
    }
}

#[get("/")]
async fn index(db: web::Data<DataBase>) -> impl Responder {
    let mut counter = db.counter.lock().unwrap();
    *counter += 1;
    format!("Counter: {counter}")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> GenericResult<()> {
    const HOST: [u8; 4] = [127, 0, 0, 1];
    const PORT: u16 = 7878;
    let addr = SocketAddr::from((HOST, PORT));

    let db = web::Data::new(DataBase::new());
    
    let server = HttpServer::new(move || {        
        App::new()
            .app_data(db.clone())
            .service(index)
            .service(hello)
    }).bind(addr)?;

    println!("Listening on http://{}", addr);

    server.run().await?;

    println!("\nServer shutdown succesful!");

    Ok(())
}
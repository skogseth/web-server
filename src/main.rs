use std::net::SocketAddr;
use actix_web::{App, HttpServer, web::Data};

mod api;
mod data;

use api::*;
use data::{DataBase, GenericResult};

#[actix_web::main]
async fn main() -> GenericResult<()> {
    const HOST: [u8; 4] = [127, 0, 0, 1];
    const PORT: u16 = 7878;
    let addr = SocketAddr::from((HOST, PORT));

    let db = Data::new(DataBase::new());
    
    let server = HttpServer::new(move || {        
        App::new()
            .app_data(db.clone())
            .service(index)
            .service(hello)
            .service(echo)
            .service(get_task)
            .service(echo_task)
    }).bind(addr)?;

    println!("Listening on http://{}", addr);

    server.run().await?;

    println!("\nServer shutdown succesful!");

    Ok(())
}
use actix_web::{get, post}; // macros
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path, Json};

use crate::data::{DataBase, TaskIdentifier};

#[get("/")]
pub async fn index(db: Data<DataBase>) -> impl Responder {
    let counter = db.increment();
    format!("Counter: {counter}")
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
// curl -X POST -d "Hello lad... " http://127.0.0.1:7878/echo


#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>) -> Json<String> {
    Json(task_identifier.into_inner().task_global_id)
}
// curl -i -X GET http://127.0.0.1:7878/task/test
// curl -i -X POST -H "Content-Type: application/json" --data "{"name": "New item", "year": "2009"}" http://127.0.0.1:7878/task

#[get("/echo/{task_global_id}")]
pub async fn echo_task(task_identifier: Path<TaskIdentifier>) -> impl Responder {
    let id = task_identifier.into_inner().task_global_id;
    HttpResponse::Ok().body(id)
}
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use tracing::{info, info_span};

use crate::repositories::{CreateTodo, TodoRepository, TodoRepositoryForMemory};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_todo);
    cfg.service(hello_todo);
}

#[post("/todos")]
pub async fn create_todo(
    payload: web::Json<CreateTodo>,
    repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    let todo = CreateTodo {
        text: payload.text.to_string(),
    };
    dbg!(&repository);
    let _span = info_span!("Post todo: ", "{:?}", todo).entered();
    info!("create_todo");
    let todo = repository.create(todo);
    HttpResponse::Ok().status(StatusCode::CREATED).json(todo)
}

#[get("/")]
pub async fn hello_todo() -> impl Responder {
    info!("Call : Hello actix!!");
    HttpResponse::Ok().body("Hello actix!!")
}

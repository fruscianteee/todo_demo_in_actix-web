use actix_web::{delete, get, http::StatusCode, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{info, info_span};

use crate::repositories::{CreateTodo, TodoRepository, TodoRepositoryForMemory};

// 各routerをここて定義する。
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_todo);
    cfg.service(all_todo);
    cfg.service(create_todo);
    cfg.service(find_todo);
    cfg.service(update_todo);
    cfg.service(delete_todo);
    cfg.service(create_user);
}
#[get("/")]
pub async fn hello_todo() -> impl Responder {
    info!("Call : Hello actix!!");
    HttpResponse::Ok().body("Hello actix!!")
}

#[get("/todos")]
pub async fn all_todo(_repository: web::Data<TodoRepositoryForMemory>) -> impl Responder {
    HttpResponse::Ok().json([1, 2])
}

#[post("/todos")]
pub async fn create_todo(
    payload: web::Json<CreateTodo>,
    repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    let todo = CreateTodo {
        text: payload.text.to_string(),
    };
    let _span = info_span!("Post todo: ", "{:?}", todo).entered();
    info!("create_todo");
    let todo = repository.create(todo);
    HttpResponse::Ok().status(StatusCode::CREATED).json(todo)
}

#[get("/todos/{id}")]
pub async fn find_todo(
    _info: web::Query<i32>,
    _repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    HttpResponse::Ok().body("Hello actix!!")
}

#[patch("/todos/{id}")]
pub async fn update_todo(
    _info: web::Query<i32>,
    _payload: web::Json<CreateTodo>,
    _repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    HttpResponse::Ok().body("Hello actix!!")
}

#[delete("/todos/{id}")]
pub async fn delete_todo(
    _info: web::Query<i32>,
    _repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    HttpResponse::Ok().body("Hello actix!!")
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub username: String,
}
#[post("/users")]
async fn create_user(payload: web::Json<CreateUser>) -> impl Responder {
    let user = User {
        id: 1337,
        username: payload.username.to_string(),
    };
    let _span = info_span!("request userdata: ", "{:?}", user).entered();
    info!("create_user");
    HttpResponse::Ok().status(StatusCode::CREATED).json(user)
}

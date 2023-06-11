use crate::repositories::{CreateTodo, TodoRepository, TodoRepositoryForMemory, UpdateTodo};
use actix_web::{
    delete, get,
    http::StatusCode,
    patch, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use tracing::{info_span, instrument};
use validator::Validate;

// 各routerをここて定義する。
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_todo);
    cfg.service(create_todo);
    cfg.service(find_todo);
    cfg.service(update_todo);
    cfg.service(delete_todo);
    cfg.service(create_user);
}

#[instrument(ret)]
#[get("/")]
pub async fn hello_todo() -> impl Responder {
    HttpResponse::Ok().body("Hello actix!!")
}

#[instrument(ret)]
#[get("/todos")]
pub async fn all_todo(repository: web::Data<TodoRepositoryForMemory>) -> impl Responder {
    let todo = repository.all().await.unwrap();
    HttpResponse::Ok().json(&todo)
}

#[instrument(ret, skip(repository))]
#[post("/todos")]
pub async fn create_todo(
    Json(payload): web::Json<CreateTodo>,
    repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    match payload.validate() {
        Ok(_) => match repository.create(payload).await {
            Ok(todo) => HttpResponse::Created().json(todo),
            Err(_) => HttpResponse::NotFound().finish(),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[instrument(ret)]
#[get("/todos/{id}")]
pub async fn find_todo(
    id: web::Path<i32>,
    repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    match repository.find(id.into_inner()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[instrument(ret)]
#[patch("/todos/{id}")]
pub async fn update_todo(
    id: web::Path<i32>,
    Json(payload): web::Json<UpdateTodo>,
    repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    match repository.update(id.into_inner(), payload).await {
        Ok(v) => HttpResponse::Created().json(v),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[instrument(ret)]
#[delete("/todos/{id}")]
pub async fn delete_todo(
    id: web::Path<i32>,
    repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    match repository.delete(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
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
#[instrument(ret)]
async fn create_user(payload: web::Json<CreateUser>) -> impl Responder {
    let user = User {
        id: 1337,
        username: payload.username.to_string(),
    };
    let _span = info_span!("request userdata: ", "{:?}", user).entered();
    HttpResponse::Ok().status(StatusCode::CREATED).json(user)
}

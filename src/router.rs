use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use thiserror::Error;
use tracing::{info, info_span};
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub username: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(root);
    cfg.service(create_user);
    cfg.service(create_todo);
}

#[get("/")]
async fn root() -> impl Responder {
    info!("Call : Hello actix!!");
    HttpResponse::Ok().body("Hello actix!!")
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

#[post("/todos")]
async fn create_todo(
    payload: web::Json<CreateTodo>,
    repository: web::Data<TodoRepositoryForMemory>,
) -> impl Responder {
    let hoge = CreateTodo {
        text: payload.text.to_string(),
    };
    dbg!(&repository);
    let todo = repository.create(hoge);
    HttpResponse::Ok().status(StatusCode::CREATED).json(todo)
}

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }
}

impl Default for TodoRepositoryForMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        todo!();
    }

    fn find(&self, id: i32) -> Option<Todo> {
        todo!();
    }
    fn all(&self) -> Vec<Todo> {
        todo!();
    }
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        todo!();
    }
    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
}

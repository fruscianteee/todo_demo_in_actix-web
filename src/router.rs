use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{info, info_span};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(root);
    cfg.service(create_user);
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub username: String,
}

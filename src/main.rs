use actix_web::{web, App, HttpServer};
use std::{env, net::SocketAddr};
use todo_demo_in_actix_web::{self, handler::config, repositories};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = env::var("RUST_LOG").unwrap_or("info".into());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);

    let repository = web::Data::new(repositories::TodoRepositoryForMemory::new());

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(repository.clone())
            .configure(config)
    })
    .bind(addr)?
    .run()
    .await
}

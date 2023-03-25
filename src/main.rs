use actix_web::{App, HttpServer};
use std::env;
use std::net::SocketAddr;
use todo_demo_in_actix_web::router;
// use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = env::var("RUST_LOG").unwrap_or("info".into());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    HttpServer::new(|| {
        App::new()
            // .wrap(TracingLogger::default())
            .configure(router::config)
    })
    .bind(addr)?
    .run()
    .await
}

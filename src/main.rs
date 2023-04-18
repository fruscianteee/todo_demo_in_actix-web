use actix_web::{web, App, HttpServer};
use std::{env, net::SocketAddr};
use todo_demo_in_actix_web::{self, handler::config, repositories};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ログレベルの設定
    let log_level = env::var("RUST_LOG").unwrap_or("info".into());
    env::set_var("RUST_LOG", log_level);

    //tracingを有効
    tracing_subscriber::fmt::init();

    // 起動する際のポートを指定する。全体へ公開するときは0.0.0.0とする。
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // デバッグモードの時のみでるログ
    tracing::debug!("listening on {}", addr);

    //データベースの初期化処理
    let repository = web::Data::new(repositories::TodoRepositoryForMemory::new());

    // actix-web起動
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default()) // ロガー
            .app_data(repository.clone()) // データベース
            .configure(config) // 各routerの定義
    })
    .bind(addr)?
    .run()
    .await
}

use actix_web::{web, App, HttpServer};
use std::{env, net::SocketAddr};
use todo_demo_in_actix_web::{self, handler::config, repositories};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // // 環境変数からログレベルを設定する。デフォルトはinfo
    // let log_level = env::var("RUST_LOG").unwrap_or("info".into());
    // env::set_var("RUST_LOG", log_level);

    //tracingを有効
    tracing_subscriber::fmt()
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .compact()
        .init();

    // 起動する際のポートを指定する。全体へ公開するときは0.0.0.0とする。
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

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

#[cfg(test)]
mod tests {
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test,
        web::{self},
        App,
    };
    use pretty_assertions::assert_eq;
    use todo_demo_in_actix_web::{
        self, handler,
        repositories::{CreateTodo, Todo, TodoRepositoryForMemory},
    };

    #[actix_web::test]
    async fn should_created_todo() {
        //初期化
        let repository = web::Data::new(TodoRepositoryForMemory::new());

        let app =
            test::init_service(App::new().app_data(repository).configure(handler::config)).await;

        let expected = Todo::new(1, "should_return_created_todo".to_string());

        let actual = CreateTodo {
            text: "should_return_created_todo".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/todos")
            .insert_header(ContentType::json())
            .set_json(&actual)
            .to_request();

        let resp: Todo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(expected, resp);
    }

    #[actix_web::test]
    async fn should_find_todo() {
        //初期化
        let repository = web::Data::new(TodoRepositoryForMemory::new());

        let app =
            test::init_service(App::new().app_data(repository).configure(handler::config)).await;

        let expected = Todo::new(1, "should_find_todo".to_string());

        let actual = CreateTodo {
            text: "should_find_todo".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .insert_header(ContentType::json())
            .set_json(&actual)
            .to_request();
        test::call_service(&app, req).await;

        let req = test::TestRequest::get().uri("/todos/1").to_request();

        let resp: Todo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(expected, resp);
    }

    #[actix_web::test]
    async fn should_get_all_todos() {
        let repository = web::Data::new(TodoRepositoryForMemory::new());
        let app =
            test::init_service(App::new().app_data(repository).configure(handler::config)).await;

        let expected = Todo::new(1, "should_get_all_todos".to_string());

        let actual = CreateTodo {
            text: "should_get_all_todos".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .insert_header(ContentType::json())
            .set_json(&actual)
            .to_request();
        test::call_service(&app, req).await;

        let req = test::TestRequest::get().uri("/todos").to_request();

        let resp: Vec<Todo> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(vec!(expected), resp);
    }

    #[actix_web::test]
    async fn should_update_todos() {
        let repository = web::Data::new(TodoRepositoryForMemory::new());
        let app =
            test::init_service(App::new().app_data(repository).configure(handler::config)).await;

        let expected = Todo::new(1, "should_update_todos".to_string());

        let actual = CreateTodo {
            text: "before_update_todos".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .insert_header(ContentType::json())
            .set_json(&actual)
            .to_request();
        test::call_service(&app, req).await;

        let update_todo = Todo::new(1, "should_update_todos".to_string());
        let req = test::TestRequest::patch()
            .uri("/todos/1")
            .insert_header(ContentType::json())
            .set_json(&update_todo)
            .to_request();

        let resp: Todo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(expected, resp);
    }

    #[actix_web::test]
    async fn should_delete_todo() {
        let repository = web::Data::new(TodoRepositoryForMemory::new());
        let app =
            test::init_service(App::new().app_data(repository).configure(handler::config)).await;

        let actual = CreateTodo {
            text: "should_delete_todos".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .insert_header(ContentType::json())
            .set_json(&actual)
            .to_request();
        test::call_service(&app, req).await;

        let req = test::TestRequest::delete().uri("/todos/1").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(StatusCode::NO_CONTENT, resp.status());
    }
}

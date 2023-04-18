#[cfg(test)]
mod tests {
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test,
        web::{self, Json},
        App,
    };
    use pretty_assertions::assert_eq;
    use todo_demo_in_actix_web::{
        self,
        handler::{self, CreateUser, User},
        repositories::{CreateTodo, Todo, TodoRepositoryForMemory},
    };

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let app =
            test::init_service(App::new().app_data(repository).configure(handler::config)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        let body_bytes = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
        let body_text = String::from_utf8_lossy(&body_bytes);

        assert_eq!("Hello actix!!", body_text);
    }

    #[actix_web::test]
    async fn test_index_post() {
        let actual = CreateUser {
            username: "田中太郎".to_string(),
        };
        let expected = User {
            id: 1337,
            username: "田中太郎".to_string(),
        };
        let app = test::init_service(App::new().configure(handler::config)).await;
        let req = test::TestRequest::post()
            .set_json(actual)
            .insert_header(ContentType::json())
            .uri("/users")
            .to_request();
        let resp: User = test::call_and_read_body_json(&app, req).await;
        assert_eq!(expected, resp);
    }

    #[tokio::test]
    async fn should_created_todo() {
        //初期化
        let repository = web::Data::new(TodoRepositoryForMemory::new());

        let app =
            test::init_service(App::new().app_data(repository).configure(handler::config)).await;

        // テストデータ作成
        let actual = CreateTodo {
            text: "hogeee".to_string(),
        };
        let expected = Todo {
            id: 1,
            text: "hogeee".to_string(),
            completed: false,
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .insert_header(ContentType::json())
            .set_json(&actual)
            .to_request();
        let resp: Todo = test::call_and_read_body_json(&app, req).await;
        // dbg!(resp.status());
        assert_eq!(expected, resp);
    }
    // レスポンスボディのアサーション
    // let body = test::read_response_json(&mut app, resp).await;
    // assert_eq!(body, expected);

    // #[tokio::test]
    // async fn should_return_user_data() {
    // let actual = CreateUser {
    //     username: "田中太郎".to_string(),
    // };

    // let expected = User {
    //     id: 1337,
    //     username: "田中太郎".to_string(),
    // };
    // let repository = TodoRepositoryForMemory::new();
    // let app = test::init_service(App::new().app_data(repository).configure(config)).await;
    // let req = test::TestRequest::post()
    //     .set_json(actual)
    //     .uri("/users")
    //     .to_request();
    // let resp = test::call_service(&app, req).await;
    // assert!(resp.status().is_success());
    // }
}

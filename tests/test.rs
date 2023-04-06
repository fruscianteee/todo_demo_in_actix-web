#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use pretty_assertions::assert_eq;
    use todo_demo_in_actix_web::{self, handler, repositories::TodoRepositoryForMemory};

    // #[actix_web::test]
    // async fn test_index_get() {
    //     let app = test::init_service(App::new().configure(config)).await;
    //     let req = test::TestRequest::get().uri("/").to_request();
    //     let resp = test::call_service(&app, req).await;
    //     assert!(resp.status().is_success());
    // }

    // #[actix_web::test]
    // async fn test_index_post() {
    //     let actual = CreateUser {
    //         username: "田中太郎".to_string(),
    //     };

    //     let expected = User {
    //         id: 1337,
    //         username: "田中太郎".to_string(),
    //     };

    //     let app = test::init_service(App::new().configure(config)).await;
    //     let req = test::TestRequest::post()
    //         .set_json(actual)
    //         .uri("/users")
    //         .to_request();
    //     let resp: User = test::call_and_read_body_json(&app, req).await;
    //     assert_eq!(expected, resp);
    // }

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

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use todo_demo_in_actix_web::router::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().configure(config)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
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

        let app = test::init_service(App::new().configure(config)).await;
        let req = test::TestRequest::post()
            .set_json(actual)
            .uri("/users")
            .to_request();
        let resp: User = test::call_and_read_body_json(&app, req).await;
        assert_eq!(expected, resp);
    }
}

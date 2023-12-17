#[cfg(test)]
mod tests {
    use crate::handlers::{Handler, R};
    use actix_web::test::{call_service, init_service, TestRequest};
    use actix_web::{http, App};

    #[actix_rt::test]
    async fn test_ip_handler_with_cf_connecting_ip() {
        let app = App::new()
            .service(actix_web::web::resource("/").route(actix_web::web::get().to(Handler::ip)));
        let mut app = init_service(app).await;

        let req = TestRequest::get()
            .uri("/")
            .insert_header(("cf-connecting-ip", "192.168.1.1"))
            .to_request();
        let resp: actix_web::dev::ServiceResponse = call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body: actix_web::web::Bytes = actix_web::test::read_body(resp).await;
        let response: R = serde_json::from_slice(&body).unwrap();

        assert_eq!(response.status, true);
        assert_eq!(response.message, "success");
        assert_eq!(response.result.unwrap().ip, "192.168.1.1");
    }

    #[actix_rt::test]
    async fn test_ip_handler_with_cf_ip_country() {
        let app = App::new()
            .service(actix_web::web::resource("/").route(actix_web::web::get().to(Handler::ip)));
        let mut app = init_service(app).await;

        let req = TestRequest::get()
            .uri("/")
            .insert_header(("cf-ipcountry", "US"))
            .to_request();
        let resp: actix_web::dev::ServiceResponse = call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body: actix_web::web::Bytes = actix_web::test::read_body(resp).await;
        let response: R = serde_json::from_slice(&body).unwrap();

        assert_eq!(response.status, true);
        assert_eq!(response.message, "success");
        assert_eq!(response.result.unwrap().ip, "US");
    }

    #[actix_rt::test]
    async fn test_ip_handler_with_x_forwarded_for() {
        let app = App::new()
            .service(actix_web::web::resource("/").route(actix_web::web::get().to(Handler::ip)));
        let mut app = init_service(app).await;

        let req = TestRequest::get()
            .uri("/")
            .insert_header(("x-forwarded-for", "192.168.0.1, 10.0.0.1"))
            .to_request();
        let resp: actix_web::dev::ServiceResponse = call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body: actix_web::web::Bytes = actix_web::test::read_body(resp).await;
        let response: R = serde_json::from_slice(&body).unwrap();

        assert_eq!(response.status, true);
        assert_eq!(response.message, "success");
        assert_eq!(response.result.unwrap().ip, "192.168.0.1");
    }

    #[actix_rt::test]
    async fn test_ip_handler_with_no_ip() {
        let app = App::new()
            .service(actix_web::web::resource("/").route(actix_web::web::get().to(Handler::ip)));
        let mut app = init_service(app).await;

        let req = TestRequest::get().uri("/").to_request();
        let resp: actix_web::dev::ServiceResponse = call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body: actix_web::web::Bytes = actix_web::test::read_body(resp).await;
        let response: R = serde_json::from_slice(&body).unwrap();

        assert_eq!(response.status, false);
        assert_eq!(response.message, "failed");
        assert_eq!(response.result, None);
    }

    #[actix_rt::test]
    async fn test_health_handler() {
        let app = App::new().service(
            actix_web::web::resource("/health").route(actix_web::web::get().to(Handler::health)),
        );
        let mut app = init_service(app).await;

        let req = TestRequest::get().uri("/health").to_request();
        let resp: actix_web::dev::ServiceResponse = call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body: actix_web::web::Bytes = actix_web::test::read_body(resp).await;
        let response: R = serde_json::from_slice(&body).unwrap();

        assert_eq!(response.status, true);
        assert_eq!(response.message, "ok");
        assert_eq!(response.result, None);
    }
}

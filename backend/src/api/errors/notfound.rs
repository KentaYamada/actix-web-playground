use actix_web::{HttpResponse, Responder};
use serde_json;

pub async fn notfound() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({"message": "Resource not found"}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test, web, App,
    };

    #[actix_web::test]
    async fn test_notfound() {
        let app = test::init_service(App::new().default_service(web::to(notfound))).await;
        let req = test::TestRequest::get()
            .uri("/api/notfound")
            .insert_header(ContentType::json())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}

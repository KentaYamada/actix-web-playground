use crate::entity::{
    request::user_view_request::UserViewRequestData,
    response::api_response::ApiResponse,
    user::User,
};
use actix_web::{Responder, Result};

pub async fn view(payload: UserViewRequestData) -> Result<impl Responder> {
    let user = User::new(payload.id, "Foo".to_string(), "Bar".to_string());
    let response = ApiResponse::new(None, Some(user));

    Ok(response.to_json())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, web, App};

    use super::*;

    #[actix_web::test]
    async fn test_view_get() {
        let app = test::init_service(App::new().route("/api/users/{id}", web::get().to(view))).await;
        let req = test::TestRequest::get()
            .uri("/api/users/1")
            .insert_header(ContentType::json())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }
}

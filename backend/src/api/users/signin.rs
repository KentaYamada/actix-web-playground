use crate::entity::{
    request::signin_request::SigninRequestData,
    response::{
        api_response::ApiResponse,
        bad_request::BadRequest,
    },
};
use actix_web::{Responder, Result};


pub async fn signin(payload: SigninRequestData) -> Result<impl Responder, BadRequest> {
    if payload.email.is_empty() {
        return Err(BadRequest { message: "Email is not empty".to_string() })
    }

    let response = ApiResponse::<()>::new(Some(format!("success {}", payload.email)), None);

    Ok(response.to_json())
}

#[cfg(test)]
mod tests {
    use crate::entity::request::signin_request::SigninRequestBody;
    use actix_web::{http::header::ContentType, test, web, App};

    use super::*;

    #[actix_web::test]
    async fn test_signin() {
        let app = test::init_service(App::new().route("/api/users/signin", web::post().to(signin))).await;
        let req = test::TestRequest::post()
            .uri("/api/users/signin")
            .insert_header(ContentType::json())
            .set_json(&SigninRequestBody {
                email: "hoge@email.com".to_string(),
                password: "hoge".to_string(),
            })
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }
}

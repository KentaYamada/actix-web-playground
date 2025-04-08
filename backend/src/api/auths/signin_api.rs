use crate::{
    entity::app_state::AppState, repository::users::fetch_user_by_password::fetch_user_by_password,
};
use actix_web::{web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SigninRequestBody {
    pub email: String,
    pub password: String,
}

pub type SigninRequest = web::Json<SigninRequestBody>;

/// Sign with password API handler
///
/// # Endpoint
/// `POST /api/auths/signin`
///
/// # Request
/// Content-Type: application/json
///
/// ```json
/// {
///    "email": "foo@email.com",
///    "password": "foobar"
/// }
/// ```
///
/// # Response
/// ```json
/// {
///    "id": 1,
///    "family_name": "Foo",
///    "first_name": "Taro",
///    "email": "foo@email.com",
///    "password": "foobar"
/// }
/// ```
///
/// # Error Response
/// ```json
/// {
///    "message": "Invalid email or password"
/// }
/// ```
/// # How to run (curl, jq)
/// ```bash
/// curl -s -v -X POST http://localhost:8080/api/auths/signin \
///      -H "content-type: application/json" \
///      -d '{ "email": "foo@email.com", "password": "foobar" }' \
///      | jq
/// ```
pub async fn signin(
    state: AppState,
    payload: SigninRequest,
) -> Result<impl Responder, actix_web::Error> {
    match fetch_user_by_password(&state.db, &payload.email, &payload.password).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound()
            .json(serde_json::json!({"message": "Invalid email or password"}))),
        Err(_) => Ok(HttpResponse::InternalServerError()
            .json(serde_json::json!({"message": "InternalServerError"}))),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{http::header::ContentType, test, web, App};
//     use dotenv::dotenv;
//     use sqlx::postgres::PgPoolOptions;
//     use std::env;
//
//     const ENDPOINT: &str = "/auths/signin";
//
//     #[actix_web::test]
//     async fn test_signin_succeded() {
//         // todo: impl build request function
//         let app = test::init_service(App::new().route(ENDPOINT, web::post().to(signin))).await;
//         let req = test::TestRequest::post()
//             .uri(ENDPOINT)
//             .insert_header(ContentType::json())
//             .to_request();
//         let res = test::call_service(&app, req).await;
//
//         assert!(res.status().is_success());
//     }
// }

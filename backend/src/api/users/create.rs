use crate::entity::{app_state::AppState, user::User};
use crate::repository::users::create_user::create_user;
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequestBody {
    pub first_name: String,
    pub family_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub message: &'static str,
    pub id: i32,
}

impl Responder for CreateUserResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type CreateUserRequest = web::Json<CreateUserRequestBody>;

/// Create user API handler
///
/// # Endpoint
/// `POST /api/users`
///
/// # Request
/// Content-Type: application/json
///
/// ```json
/// {
///    "first_name": "Foo",
///    "family_name": "Bar"
///    "email": "foo@email.com"
///    "password": "foobar"
/// }
/// ```
///
/// # Response
/// HTTP status: 200
/// ```json
/// {
///    "message": "Create successfully",
///    "id": 1
/// }
/// ```
///
/// # Error Response
/// HTTP status: 500
/// ```json
/// {
///    "message": "InternalServerError"
/// }
/// ```
/// # How to run (curl, jq)
/// ```bash
/// curl -s -v -X POST http://localhost:8080/api/users \
///      -H "content-type: application/json" \
///      -d '{ "first_name": "Foo", "family_name": "Bar", "email": "foo@email.com", "password": "foobar" }' \
///      | jq
/// ```
pub async fn create(
    state: web::Data<AppState>,
    payload: CreateUserRequest,
) -> Result<impl Responder, actix_web::Error> {
    let user = User::new(
        0,
        &payload.first_name,
        &payload.family_name,
        &payload.email,
        &payload.password,
    );

    match create_user(&state.db, &user).await {
        Ok(id) => Ok(CreateUserResponse {
            message: "Create successfully",
            id,
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

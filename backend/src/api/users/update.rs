use crate::{
    entity::{app_state::AppState, user::User},
    repository::users::update_user::update_user,
};
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequestBody {
    pub first_name: String,
    pub family_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserResponse {
    pub id: i32,
    pub mesage: &'static str,
}

impl Responder for UpdateUserResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type UpdateUserPath = web::Path<i32>;

pub type UpdateUserRequest = web::Json<UpdateUserRequestBody>;

/// Update user\ API handler
///
/// # Endpoint
/// `PUT /api/user\s/{id}`
///
/// # Parameters
/// - `id`: user id
///
/// # Request
/// Content-Type: application/json
///
/// ```json
/// {
///    "status": 0,
///    "title": "user title"
///    "detail": "description user"
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
/// curl -s -v -X PUT http://localhost:8080/api/users/1 \
///      -H "content-type: application/json" \
///      -d '{ "first_name": "Foo", "family_name": "Bar", "email": "foo@email.com", "password": "foobar" }' \
///      | jq
/// ```
pub async fn update(
    state: AppState,
    path: UpdateUserPath,
    payload: UpdateUserRequest,
) -> Result<impl Responder, actix_web::Error> {
    let user = User::new(
        path.into_inner(),
        &payload.first_name,
        &payload.family_name,
        &payload.email,
        &payload.password,
    );

    match update_user(&state.db, &user).await {
        Ok(id) => Ok(UpdateUserResponse {
            mesage: "Update successfully",
            id,
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

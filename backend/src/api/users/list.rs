use crate::entity::{app_state::AppState, user::User};
use crate::repository::users::fetch_users::fetch_users;
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserListResponse {
    pub users: Vec<User>,
}

impl Responder for UserListResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Serialize)]
pub struct UserListResponseBody {
    pub users: Vec<User>,
}

/// Search users API handler
///
/// # Endpoint
/// `GET /api/users`
///
/// # (WIP)Request
/// ```json
/// {
/// }
/// ```
///
/// # Response
/// ```json
/// {
///    "users": [
///      {
///         "id": 1,
///         "first_name": "Foo",
///         "family_name": "Bar",
///         "email": "foo@email.com",
///         "password": "foobar"
///      },
///      {
///         "id": 1,
///         "first_name": "Ham",
///         "family_name": "Spam",
///         "email": "ham@email.com",
///         "password": "hamspam"
///      }
///    ],
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
///
/// # How to run (curl, jq)
/// ```bash
/// curl -s -v -X GET http://localhost:8080/api/users -H "content-type: application/json" | jq
/// ```
pub async fn list(state: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    match fetch_users(&state.db).await {
        Ok(users) => Ok(UserListResponse { users }),
        Err(_) => Err(ErrorInternalServerError(
            serde_json::json!({ "message": "InternalServerError" }),
        )),
    }
}

use crate::{entity::app_state::AppState, repository::users::delete_user::delete_user};
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DeleteUserResponse {
    pub message: &'static str,
}

impl Responder for DeleteUserResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::NoContent()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type DeleteUserPath = web::Path<i32>;

/// Delete user API handler
///
/// # Endpoint
/// `DELETE /api/users/{id}`
///
/// # Parameters
/// - `id`: user id
///
/// # Response
/// HTTP status: 204
/// ```json
/// {
///    "message": "Deleted successfully",
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
/// curl -s -v -X DELETE http://localhost:8080/api/users/1 -H "content-type: application/json" | jq
/// ```
pub async fn delete(
    state: web::Data<AppState>,
    path: DeleteUserPath,
) -> Result<impl Responder, actix_web::Error> {
    match delete_user(&state.db, path.into_inner()).await {
        Ok(_) => Ok(DeleteUserResponse {
            message: "Deleted successfuly",
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

use crate::{
    entity::{app_state::AppState, todo::Todo},
    repository::todos::fetch_todos::fetch_todos,
};
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, HttpResponse,
    Responder, Result,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct TodoListResponse {
    pub todos: Vec<Todo>,
}

impl Responder for TodoListResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

/// Search todos API handler
///
/// # Endpoint
/// `GET /api/todos`
///
/// # (WIP)Request
/// ```json
/// {
/// }
/// ```
///
/// # Response
/// HTTP status: 200
/// ```json
/// {
///    "todos": [
///      {
///         "id": 1,
///         "status": 0,
///         "title": "todo a",
///         "detail": "todo a desctiption"
///      },
///      {
///         "id": 2,
///         "status": 1,
///         "title": "todo b",
///         "detail": "todo b desctiption"
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
/// curl -s -v -X GET http://localhost:8080/api/todos -H "content-type: application/json" | jq
/// ```
pub async fn list(state: AppState) -> Result<impl Responder, actix_web::Error> {
    match fetch_todos(&state.db).await {
        Ok(todos) => Ok(TodoListResponse { todos }),
        Err(_) => Err(ErrorInternalServerError(
            serde_json::json!({ "message": "InternalServerError" }),
        )),
    }
}

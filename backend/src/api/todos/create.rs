use crate::{
    entity::{app_state::AppState, todo::Todo},
    repository::todos::create_todo::create_todo,
};
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoRequestBody {
    pub status: i32,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTodoResponse {
    pub message: &'static str,
    pub id: i32,
}

impl Responder for CreateTodoResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type CreateTodoRequest = web::Json<CreateTodoRequestBody>;

/// Create todo API handler
///
/// # Endpoint
/// `POST /api/todos`
///
/// # Request
/// Content-Type: application/json
///
/// ```json
/// {
///    "status": 0,
///    "title": "todo title"
///    "detail": "description todo"
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
/// curl -s -v -X POST http://localhost:8080/api/todos \
///      -H "content-type: application/json" \
///      -d '{ "status": 0, "title": "todo title", "detail": "description todo" }' \
///      | jq
/// ```
pub async fn create(
    state: web::Data<AppState>,
    payload: CreateTodoRequest,
) -> Result<impl Responder, actix_web::Error> {
    let todo = Todo::new(0, payload.status, &payload.title, &payload.detail);

    match create_todo(&state.db, &todo).await {
        Ok(id) => Ok(CreateTodoResponse {
            message: "Create successfully",
            id,
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

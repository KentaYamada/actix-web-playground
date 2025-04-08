use crate::{
    entity::{app_state::AppState, todo::Todo},
    repository::todos::update_todo::update_todo,
};
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodoRequestBody {
    pub status: i32,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateTodoResponse {
    pub message: &'static str,
    pub id: i32,
}

impl Responder for UpdateTodoResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type UpdateTodoPath = web::Path<i32>;

pub type UpdateTodoRequest = web::Json<UpdateTodoRequestBody>;

/// Update todo API handler
///
/// # Endpoint
/// `PUT /api/todos/{id}`
///
/// # Parameters
/// - `id`: todo id
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
/// curl -s -v -X PUT http://localhost:8080/api/todos/1 \
///      -H "content-type: application/json" \
///      -d '{ "status": 0, "title": "todo title", "detail": "description todo" }' \
///      | jq
/// ```
pub async fn update(
    state: AppState,
    path: UpdateTodoPath,
    payload: UpdateTodoRequest,
) -> Result<impl Responder, actix_web::Error> {
    let todo = Todo::new(
        path.into_inner(),
        payload.status,
        &payload.title,
        &payload.detail,
    );

    match update_todo(&state.db, &todo).await {
        Ok(id) => Ok(UpdateTodoResponse {
            message: "Update successfully",
            id,
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

use crate::{
    entity::{app_state::AppState, todo::Todo},
    repository::todos::fetch_todos::fetch_todos,
};
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
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

pub async fn list(state: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    match fetch_todos(&state.db).await {
        Ok(todos) => Ok(TodoListResponse { todos }),
        Err(_) => Err(ErrorInternalServerError(
            serde_json::json!({ "message": "InternalServerError" }),
        )),
    }
}

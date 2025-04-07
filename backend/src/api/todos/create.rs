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

pub async fn create(
    state: web::Data<AppState>,
    payload: CreateTodoRequest,
) -> Result<impl Responder> {
    let todo = Todo::new(0, payload.status, &payload.title, &payload.detail);

    match create_todo(&state.db, &todo).await {
        Ok(id) => Ok(CreateTodoResponse {
            message: "Create successfully",
            id,
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

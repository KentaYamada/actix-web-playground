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

pub async fn update(
    state: web::Data<AppState>,
    path: UpdateTodoPath,
    payload: UpdateTodoRequest,
) -> Result<impl Responder> {
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

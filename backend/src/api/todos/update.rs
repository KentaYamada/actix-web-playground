use crate::entity::{app_state::AppState, response::api_response::ApiResponse, todo::Todo};
use crate::repository::todos::update_todo::update_todo;
use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodoRequestBody {
    pub status: i32,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateTodoResponseBody {
    pub id: i32,
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
    let id = update_todo(&state.db, &todo).await.unwrap();

    let response = ApiResponse::new(
        Some("Update successfully".to_string()),
        Some(UpdateTodoResponseBody { id }),
    );

    Ok(response.to_json())
}

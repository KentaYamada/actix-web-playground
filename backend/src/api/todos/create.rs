use crate::entity::{app_state::AppState, response::api_response::ApiResponse, todo::Todo};
use crate::repository::todos::create_todo::create_todo;
use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoRequestBody {
    pub status: i32,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTodoResponseBody {
    pub id: i32,
}

pub type CreateTodoRequest = web::Json<CreateTodoRequestBody>;

pub async fn create(
    state: web::Data<AppState>,
    payload: CreateTodoRequest,
) -> Result<impl Responder> {
    let todo = Todo::new(0, payload.status, &payload.title, &payload.detail);

    let id = create_todo(&state.db, &todo).await.unwrap();

    let response = ApiResponse::new(
        Some("Create successfully".to_string()),
        Some(CreateTodoResponseBody { id }),
    );

    Ok(response.to_json())
}

use crate::entity::{app_state::AppState, response::api_response::ApiResponse, todo::Todo};
use crate::repository::todos::fetch_todos::fetch_todos;
use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct TodoListResponseBody {
    pub todos: Vec<Todo>,
}

pub async fn list(state: web::Data<AppState>) -> Result<impl Responder> {
    let todos: Vec<Todo> = fetch_todos(&state.db).await.unwrap();
    let response = ApiResponse::new(None, Some(TodoListResponseBody { todos }));

    Ok(response.to_json())
}

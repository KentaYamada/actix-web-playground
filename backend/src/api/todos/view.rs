use crate::entity::{app_state::AppState, response::api_response::ApiResponse, todo::Todo};
use crate::repository::todos::fetch_todo_by_id::fetch_todo_by_id;
use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct TodoViewResponseBody {
    pub todo: Todo,
}

pub type TodoViewRequestPath = web::Path<i32>;

pub async fn view(state: web::Data<AppState>, path: TodoViewRequestPath) -> Result<impl Responder> {
    let todo: Todo = fetch_todo_by_id(&state.db, path.into_inner())
        .await
        .unwrap();
    let response = ApiResponse::new(None, Some(TodoViewResponseBody { todo }));

    Ok(response.to_json())
}

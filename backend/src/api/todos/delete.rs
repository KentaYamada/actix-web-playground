use crate::entity::{app_state::AppState, response::api_response::ApiResponse};
use crate::repository::todos::delete_todo::delete_todo;
use actix_web::{web, Responder, Result};

pub type DeleteTodoPath = web::Path<i32>;

pub async fn delete(state: web::Data<AppState>, path: DeleteTodoPath) -> Result<impl Responder> {
    delete_todo(&state.db, path.into_inner()).await.unwrap();

    let response = ApiResponse::<()>::new(Some("deleted successfuly".to_string()), None);

    Ok(response.to_json())
}

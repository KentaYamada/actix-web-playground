use crate::entity::{app_state::AppState, response::api_response::ApiResponse};
use crate::repository::users::delete_user::delete_user;
use actix_web::{web, Responder, Result};

pub type DeleteUserPath = web::Path<i32>;

pub async fn delete(state: web::Data<AppState>, path: DeleteUserPath) -> Result<impl Responder> {
    delete_user(&state.db, path.into_inner()).await.unwrap();

    let response = ApiResponse::<()>::new(Some("deleted successfuly".to_string()), None);

    Ok(response.to_json())
}

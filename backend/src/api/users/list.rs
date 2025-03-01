use crate::entity::{app_state::AppState, response::api_response::ApiResponse, user::User};
use crate::repository::users::fetch_users::fetch_users;
use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserListResponseBody {
    pub users: Vec<User>,
}

pub async fn list(state: web::Data<AppState>) -> Result<impl Responder> {
    let users: Vec<User> = fetch_users(&state.db).await.unwrap();
    let response = ApiResponse::new(None, Some(UserListResponseBody { users }));

    Ok(response.to_json())
}

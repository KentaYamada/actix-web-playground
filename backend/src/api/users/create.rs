use crate::entity::{app_state::AppState, response::api_response::ApiResponse, user::User};
use crate::repository::users::create_user::create_user;
use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequestBody {
    pub first_name: String,
    pub family_name: String,
    pub email: String,
    pub password: String,
}

pub type CreateUserRequest = web::Json<CreateUserRequestBody>;

pub async fn create(
    state: web::Data<AppState>,
    payload: CreateUserRequest,
) -> Result<impl Responder> {
    let user = User::new(
        0,
        payload.first_name.clone(),
        payload.family_name.clone(),
        payload.email.clone(),
        payload.password.clone(),
    );

    create_user(&state.db, &user).await.unwrap();

    let response = ApiResponse::<()>::new(Some("user created".to_string()), None);

    Ok(response.to_json())
}

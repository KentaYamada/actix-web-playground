use crate::entity::{app_state::AppState, response::api_response::ApiResponse, user::User};
use crate::repository::users::update_user::update_user;
use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequestBody {
    pub first_name: String,
    pub family_name: String,
    pub email: String,
    pub password: String,
}

pub type UpdateUserPath = web::Path<i32>;

pub type UpdateUserRequest = web::Json<UpdateUserRequestBody>;

pub async fn update(
    state: web::Data<AppState>,
    path: UpdateUserPath,
    payload: UpdateUserRequest,
) -> Result<impl Responder> {
    let user = User::new(
        path.into_inner(),
        payload.first_name.clone(),
        payload.family_name.clone(),
        payload.email.clone(),
        payload.password.clone(),
    );

    update_user(&state.db, &user).await.unwrap();

    let response = ApiResponse::<()>::new(Some("user updated".to_string()), None);

    Ok(response.to_json())
}

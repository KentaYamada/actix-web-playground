use crate::{
    entity::{app_state::AppState, user::User},
    repository::users::update_user::update_user,
};
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequestBody {
    pub first_name: String,
    pub family_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserResponse {
    pub id: i32,
    pub mesage: &'static str,
}

impl Responder for UpdateUserResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type UpdateUserPath = web::Path<i32>;

pub type UpdateUserRequest = web::Json<UpdateUserRequestBody>;

pub async fn update(
    state: web::Data<AppState>,
    path: UpdateUserPath,
    payload: UpdateUserRequest,
) -> Result<impl Responder, actix_web::Error> {
    let user = User::new(
        path.into_inner(),
        &payload.first_name,
        &payload.family_name,
        &payload.email,
        &payload.password,
    );

    match update_user(&state.db, &user).await {
        Ok(id) => Ok(UpdateUserResponse {
            mesage: "Update successfully",
            id,
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

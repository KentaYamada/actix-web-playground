use crate::entity::{app_state::AppState, user::User};
use crate::repository::users::create_user::create_user;
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequestBody {
    pub first_name: String,
    pub family_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponseBody {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub message: &'static str,
    pub id: i32,
}

impl Responder for CreateUserResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type CreateUserRequest = web::Json<CreateUserRequestBody>;

pub async fn create(
    state: web::Data<AppState>,
    payload: CreateUserRequest,
) -> Result<impl Responder, actix_web::Error> {
    let user = User::new(
        0,
        &payload.first_name,
        &payload.family_name,
        &payload.email,
        &payload.password,
    );

    match create_user(&state.db, &user).await {
        Ok(id) => Ok(CreateUserResponse {
            message: "Create successfully",
            id,
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

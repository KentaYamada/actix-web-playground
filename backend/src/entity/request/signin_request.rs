use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SigninRequestBody {
    pub email: String,

    pub password: String,
}

pub type SigninRequestData = web::Json<SigninRequestBody>;

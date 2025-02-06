use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserViewRequestBody {
    pub id: u64,
}

pub type UserViewRequestData = web::Path<UserViewRequestBody>;

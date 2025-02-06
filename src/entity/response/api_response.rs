use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: Option<String>,

    pub data: Option<T>,
}

impl <T> ApiResponse<T> {
    pub fn new(message: Option<String>, data: Option<T>) -> Self {
        Self {
            message,
            data,
        }
    }

    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

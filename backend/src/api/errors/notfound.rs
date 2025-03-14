use actix_web::{HttpResponse, Responder};
use serde_json;

pub async fn notfound() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({"message": "Resource not found"}))
}

use crate::{entity::app_state::AppState, repository::todos::delete_todo::delete_todo};
use actix_web::{
    body::BoxBody, error::ErrorInternalServerError, http::header::ContentType, web, HttpResponse,
    Responder, Result,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DeleteTodoResponse {
    pub message: &'static str,
}

impl Responder for DeleteTodoResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type DeleteTodoPath = web::Path<i32>;

pub async fn delete(
    state: web::Data<AppState>,
    path: DeleteTodoPath,
) -> Result<impl Responder, actix_web::Error> {
    match delete_todo(&state.db, path.into_inner()).await {
        Ok(_) => Ok(DeleteTodoResponse {
            message: "Deleted successfuly",
        }),
        Err(_) => Err(ErrorInternalServerError("InternalServerError")),
    }
}

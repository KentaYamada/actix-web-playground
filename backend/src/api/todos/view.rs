use crate::{
    entity::{app_state::AppState, todo::Todo},
    repository::todos::fetch_todo_by_id::fetch_todo_by_id,
};
use actix_web::{
    body::BoxBody,
    error::{ErrorInternalServerError, ErrorNotFound},
    http::header::ContentType,
    web, HttpResponse, Responder, Result,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct TodoViewResponse {
    pub todo: Todo,
}

impl Responder for TodoViewResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type TodoViewRequestPath = web::Path<i32>;

pub async fn view(state: web::Data<AppState>, path: TodoViewRequestPath) -> Result<impl Responder> {
    match fetch_todo_by_id(&state.db, path.into_inner()).await {
        Ok(Some(todo)) => Ok(TodoViewResponse { todo }),
        Ok(None) => Err(ErrorNotFound(
            serde_json::json!({ "message": "User not found." }),
        )),
        Err(_) => Err(ErrorInternalServerError(
            serde_json::json!({ "message": "InternalServerError"}),
        )),
    }
}

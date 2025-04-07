use crate::{
    entity::{app_state::AppState, user::User},
    repository::users::fetch_user_by_id::fetch_user_by_id,
};
use actix_web::{
    body::BoxBody,
    error::{ErrorInternalServerError, ErrorNotFound},
    http::header::ContentType,
    web, HttpResponse, Responder, Result,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserViewResponse {
    pub user: User,
}

impl Responder for UserViewResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type UserViewRequestPath = web::Path<i32>;

pub async fn view(
    state: web::Data<AppState>,
    path: UserViewRequestPath,
) -> Result<impl Responder, actix_web::Error> {
    match fetch_user_by_id(&state.db, path.into_inner()).await {
        Ok(Some(user)) => Ok(UserViewResponse { user }),
        Ok(None) => Err(ErrorNotFound(
            serde_json::json!({ "message": "User not found." }),
        )),
        Err(_) => Err(ErrorInternalServerError(
            serde_json::json!({ "message": "InternalServerError"}),
        )),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, web, App};

    use super::*;

    #[actix_web::test]
    #[ignore]
    async fn test_view_get() {
        let app =
            test::init_service(App::new().route("/api/users/{id}", web::get().to(view))).await;
        let req = test::TestRequest::get()
            .uri("/api/users/1")
            .insert_header(ContentType::json())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }
}

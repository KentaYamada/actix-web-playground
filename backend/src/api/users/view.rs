use crate::entity::{app_state::AppState, response::api_response::ApiResponse, user::User};
use crate::repository::users::fetch_user_by_id::fetch_user_by_id;
use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserViewResponseBody {
    pub user: Option<User>,
}

pub type UserViewRequestPath = web::Path<i32>;

pub async fn view(state: web::Data<AppState>, path: UserViewRequestPath) -> Result<impl Responder> {
    let user: Option<User> = fetch_user_by_id(&state.db, path.into_inner())
        .await
        .expect("Failed fetch user");

    // if user == None {
    //     Err(HttpResponse::NotFound().json("notfound"));
    // }

    let response = ApiResponse::new(None, Some(UserViewResponseBody { user }));

    Ok(response.to_json())
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

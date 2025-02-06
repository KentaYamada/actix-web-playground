mod api;
mod entity;

use crate::api::users::{
    signin::signin,
    view::view,
};
use actix_web::{guard, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .guard(guard::Header("content-type", "application/json"))
                    .route("/users/{id}", web::get().to(view))
                    .route("/users/signin", web::post().to(signin))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

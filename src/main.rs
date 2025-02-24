mod api;
mod entity;

use actix_web::{guard, web, App, HttpServer};
use api::users::{signin::signin, view::view};
use dotenv::dotenv;
use entity::app_state::AppState;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("[Fatal] DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("[Fatal] Failed build connection pools.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(
                web::scope("/api")
                    .guard(guard::Header("content-type", "application/json"))
                    .route("/users/{id}", web::get().to(view))
                    .route("/users/signin", web::post().to(signin)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

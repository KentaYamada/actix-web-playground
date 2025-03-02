mod api;
mod entity;
mod repository;

use actix_web::{guard, web, App, HttpServer};
use api::{
    todos,
    users::{
        create::create, delete::delete, list::list, signin::signin, update::update, view::view,
    },
};
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
                    .route("/users", web::post().to(create))
                    .route("/users", web::get().to(list))
                    .route("/users/{id}", web::get().to(view))
                    .route("/users/{id}", web::patch().to(update))
                    .route("/users/{id}", web::delete().to(delete))
                    .route("/users/signin", web::post().to(signin))
                    .route("/todos", web::post().to(todos::create::create))
                    .route("/todos/{id}", web::patch().to(todos::update::update)),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

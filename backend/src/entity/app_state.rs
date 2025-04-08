use sqlx::{Pool, Postgres};

pub struct AppData {
    pub db: Pool<Postgres>,
}

pub type AppState = actix_web::web::Data<AppData>;

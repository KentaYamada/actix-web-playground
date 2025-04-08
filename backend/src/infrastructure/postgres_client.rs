use sqlx::{postgres::PgPoolOptions, PgPool};

pub type PostgreConnectionResult = Result<PgPool, sqlx::Error>;

pub async fn init_connection_pool(url: &str, capacity: u32) -> PostgreConnectionResult {
    let pool = PgPoolOptions::new()
        .max_connections(capacity)
        .connect(&url)
        .await?;

    Ok(pool)
}

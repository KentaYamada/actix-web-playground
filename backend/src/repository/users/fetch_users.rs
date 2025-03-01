use crate::entity::user::User;
use sqlx::{Error, PgPool};

pub async fn fetch_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users LIMIT 100")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

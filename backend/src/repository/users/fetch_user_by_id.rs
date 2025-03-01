use crate::entity::user::User;
use sqlx::{Error, PgPool};

pub async fn fetch_user_by_id(pool: &PgPool, id: i32) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

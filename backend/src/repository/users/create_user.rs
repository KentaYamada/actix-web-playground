use crate::entity::user::User;
use sqlx::{Error, PgPool};

/// ユーザー新規作成
pub async fn create_user(pool: &PgPool, user: &User) -> Result<i32, Error> {
    let row = sqlx::query_as::<_, (i32,)>("INSERT INTO users (first_name, family_name, email, password) VALUES ($1, $2, $3, $4) RETURNING id")
        .bind(&user.first_name)
        .bind(&user.family_name)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(pool)
        .await?;
    let id = row.0;

    Ok(id)
}

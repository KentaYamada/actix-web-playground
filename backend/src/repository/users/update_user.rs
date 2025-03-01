use crate::entity::user::User;
use sqlx::{Error, PgPool};

/// ユーザー情報更新
pub async fn update_user(pool: &PgPool, user: &User) -> Result<i32, Error> {
    let row = sqlx::query_as::<_, (i32,)>("UPDATE users SET first_name = $1, family_name = $2, email = $3, password = $4 WHERE id = $5 RETURNING id")
    .bind(&user.first_name)
    .bind(&user.family_name)
    .bind(&user.email)
    .bind(&user.password)
    .bind(user.id)
    .fetch_one(pool)
    .await?;
    let id = row.0;

    Ok(id)
}

use sqlx::{Error, PgPool};

/// ユーザー削除
pub async fn delete_user(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

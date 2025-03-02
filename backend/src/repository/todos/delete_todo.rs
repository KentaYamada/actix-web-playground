use sqlx::{Error, PgPool};

pub async fn delete_todo(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

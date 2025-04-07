use crate::entity::sqlx_result::SqlxResult;
use sqlx::PgPool;

pub async fn delete_todo(pool: &PgPool, id: i32) -> SqlxResult<()> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

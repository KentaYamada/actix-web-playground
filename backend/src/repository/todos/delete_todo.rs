use crate::entity::sqlx_result::SqlxResult;
use sqlx::PgPool;

pub async fn delete_todo(pool: &PgPool, id: i32) -> SqlxResult<()> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::todo::Todo;
    use sqlx::{query_as, PgPool};

    #[sqlx::test(fixtures("todo_fixture"))]
    async fn test_delete_todo(pool: PgPool) {
        let todo_id = 1;
        delete_todo(&pool, todo_id)
            .await
            .expect("Failed to delete todo");

        let todo = query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
            .bind(todo_id)
            .fetch_optional(&pool)
            .await
            .expect("Failed to fetch todo");

        assert!(todo.is_none());
    }
}

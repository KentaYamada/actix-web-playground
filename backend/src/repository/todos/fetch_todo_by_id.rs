use crate::entity::todo::Todo;
use sqlx::{Error, PgPool};

pub async fn fetch_todo_by_id(pool: &PgPool, id: i32) -> Result<Option<Todo>, Error> {
    let todo =
        sqlx::query_as::<_, Todo>("SELECT id, status, title, detail FROM todos WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    Ok(todo)
}

#[cfg(test)]
mod tests {

    use super::*;
    use sqlx::PgPool;

    #[sqlx::test(fixtures("todos_fixture"))]
    async fn test_fetch_todo(pool: PgPool) {
        let todo = fetch_todo_by_id(&pool, 1)
            .await
            .expect("Failed to fetch todo")
            .expect("User not found.");

        assert_eq!(todo.status, 0);
    }

    #[sqlx::test()]
    async fn test_fetch_no_todo(pool: PgPool) {
        let todo = fetch_todo_by_id(&pool, 1)
            .await
            .expect("Failed to fetch todo");

        assert!(todo.is_none());
    }
}

use crate::entity::{sqlx_result::SqlxResult, todo::Todo};
use sqlx::PgPool;

pub async fn fetch_todos(pool: &PgPool) -> SqlxResult<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos ORDER BY status asc, modified_at desc LIMIT 100",
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

#[cfg(test)]
mod tests {

    use super::*;
    use sqlx::PgPool;

    #[sqlx::test(fixtures("todos_fixture"))]
    async fn test_fetch_todos(pool: PgPool) {
        let todos = fetch_todos(&pool).await.expect("Failed to fetch todos");

        assert_eq!(todos.iter().count(), 3);
    }

    #[sqlx::test()]
    async fn test_fetch_no_todo(pool: PgPool) {
        let todos = fetch_todos(&pool).await.expect("Failed to fetch todos");

        assert_eq!(todos.iter().count(), 0);
    }
}

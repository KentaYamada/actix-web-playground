use crate::entity::todo::Todo;
use sqlx::{Error, PgPool};

pub async fn fetch_todos(pool: &PgPool) -> Result<Vec<Todo>, Error> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos ORDER BY status asc, modified_at desc LIMIT 100",
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

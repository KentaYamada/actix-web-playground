use crate::entity::todo::Todo;
use sqlx::{Error, PgPool};

pub async fn fetch_todo_by_id(pool: &PgPool, id: i32) -> Result<Todo, Error> {
    let todo =
        sqlx::query_as::<_, Todo>("SELECT id, status, title, detail FROM todos WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

    Ok(todo)
}

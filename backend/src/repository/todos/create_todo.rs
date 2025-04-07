use crate::entity::{sqlx_result::SqlxResult, todo::Todo};
use chrono::{DateTime, Local};
use sqlx::PgPool;

pub async fn create_todo(pool: &PgPool, todo: &Todo) -> SqlxResult<i32> {
    let now: DateTime<Local> = Local::now();
    let row = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO todos (status, title, detail, created_at, modified_at) VALUES ($1, $2, $3, $4, $5) RETURNING id",
    )
    .bind(todo.status)
    .bind(&todo.title)
    .bind(&todo.detail)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;
    let id = row.0;

    Ok(id)
}

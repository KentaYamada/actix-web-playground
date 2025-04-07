use crate::entity::{sqlx_result::SqlxResult, todo::Todo};
use chrono::{DateTime, Local};
use sqlx::PgPool;

pub async fn update_todo(pool: &PgPool, todo: &Todo) -> SqlxResult<i32> {
    let now: DateTime<Local> = Local::now();
    let row = sqlx::query_as::<_, (i32,)>("UPDATE todos SET status = $1, title = $2, detail = $3, modified_at = $4 WHERE id = $5 RETURNING id")
                .bind(todo.status)
                .bind(&todo.title)
                .bind(&todo.detail)
                .bind(now)
                .bind(todo.id)
                .fetch_one(pool)
                .await?;
    let id = row.0;

    Ok(id)
}

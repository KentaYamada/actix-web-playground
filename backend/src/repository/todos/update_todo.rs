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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::todo::Todo;
    use sqlx::{query_as, PgPool};

    #[sqlx::test(fixtures("todo_fixture"))]
    async fn test_update_todo_succeeded(pool: PgPool) {
        let todo_id = 1;
        let todo = Todo::new(todo_id, 1, "Test", "updated");
        let updated_id = update_todo(&pool, &todo)
            .await
            .expect("Failed to update todo");

        assert_eq!(todo_id, updated_id);

        let updated_todo = query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
            .bind(todo_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch todo");

        assert_eq!(todo.status, updated_todo.status);
        assert_eq!(todo.title, updated_todo.title);
        assert_eq!(todo.detail, updated_todo.detail);
    }
}

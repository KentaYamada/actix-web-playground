use sqlx::{Error, PgPool};

/// ユーザー削除
pub async fn delete_user(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::user::User;
    use sqlx::{query_as, PgPool};

    #[sqlx::test(fixtures("users"))]
    async fn test_delete_user(pool: PgPool) {
        let user_id = 1;

        delete_user(&pool, user_id)
            .await
            .expect("Failed delete user");

        let user = query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&pool)
            .await
            .expect("Failed fetch user");

        assert!(user.is_none());
    }
}

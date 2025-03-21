use crate::entity::user::User;
use sqlx::PgPool;

pub async fn fetch_user_by_password(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as("SELECT * FROM users WHERE email = $1 AND password = $2")
        .bind(email)
        .bind(password)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test(fixtures("update_user_fixture"))]
    async fn test_fetch_user_by_password(pool: PgPool) {
        let user = fetch_user_by_password(&pool, "hoge.taro@email.com", "foobar")
            .await
            .expect("Failed to fetch user");

        assert!(user.is_some());
    }

    #[sqlx::test()]
    async fn test_fetch_user_by_password_no_user(pool: PgPool) {
        let user = fetch_user_by_password(&pool, "hoge.taro@email.com", "foobar")
            .await
            .expect("Failed to fetch user");

        assert!(user.is_none());
    }
}

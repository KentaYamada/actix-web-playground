use crate::entity::{sqlx_result::SqlxResult, user::User};
use sqlx::PgPool;

pub async fn fetch_users(pool: &PgPool) -> SqlxResult<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users LIMIT 100")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test(fixtures("users"))]
    async fn test_fetch_users(pool: PgPool) {
        let users = fetch_users(&pool).await.expect("Failed to fetch users");

        assert_eq!(users.iter().count(), 3);
    }

    #[sqlx::test()]
    async fn test_fetch_no_user(pool: PgPool) {
        let users = fetch_users(&pool).await.expect("Failed to fetch users");

        assert_eq!(users.iter().count(), 0);
    }
}

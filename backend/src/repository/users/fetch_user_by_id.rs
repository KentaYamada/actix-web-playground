use crate::entity::user::User;
use sqlx::{Error, PgPool};

pub async fn fetch_user_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test(fixtures("users"))]
    async fn test_fetch_user_by_id(pool: PgPool) {
        let user = fetch_user_by_id(&pool, 1)
            .await
            .expect("Failed to fetch account.")
            .expect("User not found.");

        assert_eq!(user.family_name, "hoge");
        assert_eq!(user.first_name, "taro");
        assert_eq!(user.email, "hoge.taro@email.com");
    }

    #[sqlx::test()]
    async fn test_fetch_user_not_found(pool: PgPool) {
        let user = fetch_user_by_id(&pool, 0)
            .await
            .expect("Failed to fetch account.");

        assert!(user.is_none());
    }
}

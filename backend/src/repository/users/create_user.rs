use crate::entity::{sqlx_result::SqlxResult, user::User};
use sqlx::PgPool;

/// ユーザー新規作成
pub async fn create_user(pool: &PgPool, user: &User) -> SqlxResult<i32> {
    let row = sqlx::query_as::<_, (i32,)>("INSERT INTO users (first_name, family_name, email, password) VALUES ($1, $2, $3, $4) RETURNING id")
        .bind(&user.first_name)
        .bind(&user.family_name)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(pool)
        .await?;
    let id = row.0;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::user::User;
    use sqlx::PgPool;

    #[sqlx::test()]
    async fn test_create_user_succeeded(pool: PgPool) {
        let user = User::new(0, "Hoge", "Fuga", "hoge.fuga@email.com", "hogefuga");
        let id = create_user(&pool, &user).await.expect("Failed create user");

        assert!(id > 0)
    }
}

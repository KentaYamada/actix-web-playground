use crate::entity::user::User;
use sqlx::{Error, PgPool};

/// ユーザー情報更新
pub async fn update_user(pool: &PgPool, user: &User) -> Result<i32, Error> {
    let row = sqlx::query_as::<_, (i32,)>("UPDATE users SET first_name = $1, family_name = $2, email = $3, password = $4 WHERE id = $5 RETURNING id")
    .bind(&user.first_name)
    .bind(&user.family_name)
    .bind(&user.email)
    .bind(&user.password)
    .bind(user.id)
    .fetch_one(pool)
    .await?;
    let id = row.0;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::user::User;
    use sqlx::{query_as, PgPool};

    #[sqlx::test(fixtures("update_user_fixture"))]
    async fn test_update_user(pool: PgPool) {
        let user_id: i32 = 1;
        let user = User::new(user_id, "Hanako", "Foo", "hanaako.foo@email.com", "foobar");
        let id = update_user(&pool, &user).await.expect("Failed update user");

        assert_eq!(user_id, id);

        let updated_user = query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .expect("Failed fetch updated user");

        assert_eq!(user.first_name, updated_user.first_name);
        assert_eq!(user.family_name, updated_user.family_name);
        assert_eq!(user.email, updated_user.email);
        assert_eq!(user.password, updated_user.password);
    }
}

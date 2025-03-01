use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub family_name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(
        id: i32,
        first_name: String,
        family_name: String,
        email: String,
        password: String,
    ) -> Self {
        Self {
            id,
            first_name,
            family_name,
            email,
            password,
        }
    }
}

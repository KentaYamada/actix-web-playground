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
    pub fn new(id: i32, first_name: &str, family_name: &str, email: &str, password: &str) -> Self {
        Self {
            id,
            first_name: first_name.to_string(),
            family_name: family_name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

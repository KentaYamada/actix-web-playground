use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Todo {
    pub id: i32,
    pub status: i32,
    pub title: String,
    pub detail: String,
}

impl Todo {
    pub fn new(id: i32, status: i32, title: &str, detail: &str) -> Self {
        Self {
            id,
            status,
            title: title.to_string(),
            detail: detail.to_string(),
        }
    }
}

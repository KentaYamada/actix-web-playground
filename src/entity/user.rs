use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: u64,

    pub last_name: String,

    pub family_name: String,
}

impl User {
    pub fn new(id: u64, last_name: String, family_name: String) -> Self {
        Self {
            id,
            last_name,
            family_name,
        }
    }
}

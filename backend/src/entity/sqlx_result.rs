use std::result;

pub type SqlxResult<T> = result::Result<T, sqlx::Error>;

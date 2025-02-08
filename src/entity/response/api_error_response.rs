use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiErrorResponseBody {
    message: String,
}

impl ApiErrorResponseBody {
    pub fn new(message: String) -> Self{
        Self {
            message
        }
    }
}

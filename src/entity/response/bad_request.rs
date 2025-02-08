use crate::entity::response::api_error_response::ApiErrorResponseBody;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display("BadRequest: {message}")]
pub struct BadRequest {
    pub message: String,
}

impl ResponseError for BadRequest {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiErrorResponseBody::new(self.message.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    // use actix_web::{test, http::StatusCode};
    // use serde_json::{json, Value};

    #[actix_web::test]
    async fn test_bad_request_status_code() {
        let error = BadRequest {
            message: "Invalid input".to_string(),
        };
        assert_eq!(error.status_code(), StatusCode::BAD_REQUEST);
    }

    // #[actix_web::test]
    // async fn test_bad_request_error_response() {
    //     let error = BadRequest {
    //         message: "Invalid input".to_string(),
    //     };
    //     let response = error.error_response();

    //     assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    //     // todo: check response body
    // }
}

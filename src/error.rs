use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let error_body = json!({
            "error": {
                "type": self.status.as_u16(),
                "message": self.message
            }
        });

        (self.status, Json(error_body)).into_response()
    }
}

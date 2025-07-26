use axum::{Json, extract::rejection::JsonRejection, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractorError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for ExtractorError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ExtractorError::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            }
        };

        (status, Json(json!({"message" : message}))).into_response()
    }
}

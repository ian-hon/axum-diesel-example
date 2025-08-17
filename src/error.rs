use std::error::Error;
use std::fmt;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, extract};
use serde_json::json;
use tracing::error;

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

#[derive(Debug)]
pub struct JsonRejection(extract::rejection::JsonRejection);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to
// turn them into `Result<_, AppError>`. That way you don't need to do that
// manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!(err = ?self.0, "app error");
        // Don't expose sensitive error details to the client.
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"title": "ServerError", "detail": "Something went wrong"})),
        )
            .into_response()
    }
}

impl fmt::Display for JsonRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{json_rejection}", json_rejection = self.0)
    }
}

impl Error for JsonRejection {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl From<extract::rejection::JsonRejection> for JsonRejection {
    fn from(json_rejection: extract::rejection::JsonRejection) -> Self {
        Self(json_rejection)
    }
}

impl IntoResponse for JsonRejection {
    fn into_response(self) -> Response {
        let json_rejection = self.0;

        (
            json_rejection.status(),
            Json(json!({"title": "InvalidRequest", "detail": json_rejection.body_text()})),
        )
            .into_response()
    }
}

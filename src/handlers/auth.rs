use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use crate::extractor_error::ExtractorError;
use crate::state::AppState;

// The request body will be parsed into this struct
// Example body:
// {
// "username": "lorem_ipsum",
// "password": "foo_bar"
// }
#[derive(Serialize, Deserialize, Debug)]
pub struct RawRequestBody {
    username: String,
    password: String,
}

pub async fn post_login(
    State(state): State<AppState>,
    WithRejection(Json(request_body), _): WithRejection<Json<RawRequestBody>, ExtractorError>,
) -> impl IntoResponse {
    // 1. Check if user exists
    if let Some(fetched_password) = state.user_data.lock().await.get(&request_body.username) {
        // 2. Check if password matches
        if *fetched_password == request_body.password {
            return "Success".to_string();
        }
        return "PasswordWrong".to_string();
    }
    "UserNotFound".to_string()
}

pub async fn post_signup(
    State(state): State<AppState>,
    WithRejection(Json(request_body), _): WithRejection<Json<RawRequestBody>, ExtractorError>,
) -> impl IntoResponse {
    // 1. Check if user exists
    match state
        .user_data
        .lock()
        .await
        .insert(request_body.username, request_body.password)
    {
        Some(_) => "UsernameTaken".to_string(), // Username already exists
        None => "Success".to_string(),
    }
}

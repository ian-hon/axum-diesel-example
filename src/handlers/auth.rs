use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::WithRejection;

use crate::extractor_error::ExtractorError;
use crate::models::auth_body::AuthBody;
use crate::models::user;
use crate::state::AppState;

pub async fn post_login(
    State(state): State<AppState>,
    WithRejection(Json(auth_body), _): WithRejection<Json<AuthBody>, ExtractorError>,
) -> impl IntoResponse {
    match auth_body.validate(&state).await {
        Ok(_) => "Success".to_string(),
        Err(e) => serde_json::to_string(&e).unwrap(),
    }
}

pub async fn post_signup(
    State(state): State<AppState>,
    WithRejection(Json(auth_body), _): WithRejection<Json<AuthBody>, ExtractorError>,
) -> impl IntoResponse {
    match user::create(&state, auth_body).await {
        Ok(_) => "Sucess".to_string(),
        Err(e) => serde_json::to_string(&e).unwrap(),
    }
}

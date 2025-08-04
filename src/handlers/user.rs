use axum::{Json, extract::State, response::IntoResponse};
use axum_extra::extract::WithRejection;

use crate::{extractor_error::ExtractorError, models::auth_body::AuthBody, state::AppState};

pub async fn post_fetch_balance(
    State(state): State<AppState>,
    WithRejection(Json(auth), _): WithRejection<Json<AuthBody>, ExtractorError>,
) -> impl IntoResponse {
    match auth.validate(&state).await {
        Ok(u) => u.balance.to_string(),
        Err(e) => serde_json::to_string(&e).unwrap(),
    }
}

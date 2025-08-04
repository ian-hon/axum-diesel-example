use axum::Json;
use axum::extract::Query;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::extractor_error::ExtractorError;
use crate::models::auth_body::AuthBody;
use crate::models::transactions;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct TransferParams {
    target: String,
    amount: f64,
}

pub async fn post_transfer(
    State(state): State<AppState>,
    Query(transfer): Query<TransferParams>,
    WithRejection(Json(auth), _): WithRejection<Json<AuthBody>, ExtractorError>,
) -> impl IntoResponse {
    match auth.validate(&state).await {
        Ok(u) => {
            match transactions::transfer_amount(
                &state,
                transfer.amount,
                &transfer.target,
                &u.username,
            )
            .await
            {
                Ok(_) => "Success".to_string(),
                Err(e) => serde_json::to_string(&e).unwrap(),
            }
        },
        Err(e) => serde_json::to_string(&e).unwrap(),
    }
}

pub async fn post_fetch_all(
    State(state): State<AppState>,
    WithRejection(Json(auth), _): WithRejection<Json<AuthBody>, ExtractorError>,
) -> impl IntoResponse {
    match auth.validate(&state).await {
        Ok(u) => {
            serde_json::to_string(&transactions::fetch_all(&state, &u.username).await).unwrap()
        },
        Err(e) => serde_json::to_string(&e).unwrap(),
    }
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Context as _;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Result;
use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::error::{AppError, JsonRejection};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{Transaction, User};

#[derive(Deserialize)]
pub struct PostTranscactionPayload {
    #[serde(with = "bigdecimal::serde::json_num")]
    amount: BigDecimal,
    recipient: Uuid,
    sender: Uuid,
}

#[derive(Serialize)]
pub struct PostTransactionResponse {
    id: Uuid,
    #[serde(with = "bigdecimal::serde::json_num")]
    amount: BigDecimal,
    recipient: Uuid,
    sender: Uuid,
    timestamp: jiff::Timestamp,
}

pub async fn post_transaction(
    State(users): State<Arc<Mutex<HashMap<Uuid, User>>>>,
    State(transactions): State<Arc<Mutex<HashMap<Uuid, Transaction>>>>,
    Extension(authenticated_user): Extension<AuthenticatedUser>,
    WithRejection(Json(payload), _): WithRejection<Json<PostTranscactionPayload>, JsonRejection>,
) -> Result<Json<PostTransactionResponse>> {
    if authenticated_user.subject != payload.sender {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "title": "PermissionDenied",
            })),
        ))?;
    }

    let mut users = users.lock().unwrap();
    let user = users
        .get_mut(&payload.sender)
        .context("could not find user")
        .map_err(AppError::from)?;

    if user.balance < payload.amount {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "title": "InsufficientBalance",
            })),
        ))?;
    }

    // Start of transaction boundary

    let mut transactions = transactions.lock().unwrap();

    let transaction_id = Uuid::now_v7();
    let transaction = Transaction {
        id: transaction_id,
        amount: payload.amount,
        recipient: payload.recipient,
        sender: payload.sender,
        timestamp: jiff::Timestamp::now(),
    };
    transactions.insert(transaction_id, transaction);
    let transaction = transactions.get(&transaction_id).unwrap();

    user.balance -= &transaction.amount;

    // End of transaction boundary

    Ok(Json(PostTransactionResponse {
        id: transaction_id,
        amount: transaction.amount.clone(),
        recipient: transaction.recipient,
        sender: transaction.sender,
        timestamp: transaction.timestamp,
    }))
}

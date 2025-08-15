use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Context as _;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Result;
use axum::{Extension, Json};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{Transaction, User};

#[derive(Deserialize)]
pub struct GetUserPathParams {
    user_id: Uuid,
}

#[derive(Serialize)]
pub struct GetUserResponse {
    id: Uuid,
    username: String,
    #[serde(with = "bigdecimal::serde::json_num")]
    balance: BigDecimal,
}

#[derive(Deserialize)]
pub struct GetTransactionsPathParams {
    user_id: Uuid,
}

#[derive(Serialize)]
pub struct GetTransactionsResponse {
    transactions: Vec<TransactionResponse>,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    id: Uuid,
    #[serde(with = "bigdecimal::serde::json_num")]
    amount: BigDecimal,
    recipient: Uuid,
    sender: Uuid,
    timestamp: jiff::Timestamp,
}

pub async fn get_user(
    State(users): State<Arc<Mutex<HashMap<Uuid, User>>>>,
    Extension(authenticated_user): Extension<AuthenticatedUser>,
    Path(GetUserPathParams { user_id }): Path<GetUserPathParams>,
) -> Result<Json<GetUserResponse>> {
    if authenticated_user.subject != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "title": "PermissionDenied",
            })),
        ))?;
    }

    let users = users.lock().unwrap();
    let user = users
        .get(&user_id)
        .context("could not find user")
        .map_err(AppError::from)?;

    Ok(Json(GetUserResponse {
        id: user.id,
        username: user.username.clone(),
        balance: user.balance.clone(),
    }))
}

pub async fn get_transactions(
    State(transactions): State<Arc<Mutex<HashMap<Uuid, Transaction>>>>,
    Extension(authenticated_user): Extension<AuthenticatedUser>,
    Path(GetTransactionsPathParams { user_id }): Path<GetTransactionsPathParams>,
) -> Result<Json<GetTransactionsResponse>> {
    if authenticated_user.subject != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "title": "PermissionDenied",
            })),
        ))?;
    }

    let transactions = transactions.lock().unwrap();

    Ok(Json(GetTransactionsResponse {
        transactions: transactions
            .values()
            .map(|transaction| TransactionResponse {
                id: transaction.id,
                amount: transaction.amount.clone(),
                recipient: transaction.recipient,
                sender: transaction.sender,
                timestamp: transaction.timestamp,
            })
            .collect(),
    }))
}

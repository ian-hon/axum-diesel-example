use anyhow::Context as _;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Result;
use axum::{Extension, Json};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
#[allow(
    clippy::unused_trait_names,
    reason = "error[E0034]: multiple applicable items in scope"
)]
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{Transaction, User};
use crate::state::DbConnectionPool;

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
    State(pool): State<DbConnectionPool>,
    Extension(authenticated_user): Extension<AuthenticatedUser>,
    Path(GetUserPathParams { user_id }): Path<GetUserPathParams>,
) -> Result<Json<GetUserResponse>> {
    use crate::models::types;
    use crate::schema::users;

    let mut conn = pool
        .get()
        .await
        .context("failed to get database connection")
        .map_err(AppError::from)?;

    if authenticated_user.subject != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "title": "PermissionDenied",
            })),
        ))?;
    }

    let user: User = users::table
        .find(types::Uuid::from(user_id))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .context("could not find user")
        .map_err(AppError::from)?;

    Ok(Json(GetUserResponse {
        id: user.id,
        username: user.username,
        balance: user.balance,
    }))
}

pub async fn get_transactions(
    State(pool): State<DbConnectionPool>,
    Extension(authenticated_user): Extension<AuthenticatedUser>,
    Path(GetTransactionsPathParams { user_id }): Path<GetTransactionsPathParams>,
) -> Result<Json<GetTransactionsResponse>> {
    use crate::models::types;
    use crate::schema::transactions;

    let mut conn = pool
        .get()
        .await
        .context("failed to get database connection")
        .map_err(AppError::from)?;

    if authenticated_user.subject != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "title": "PermissionDenied",
            })),
        ))?;
    }

    let transactions: Vec<Transaction> = transactions::table
        .filter(
            transactions::recipient
                .eq(types::Uuid::from(user_id))
                .or(transactions::sender.eq(types::Uuid::from(user_id))),
        )
        .select(Transaction::as_select())
        .order(transactions::timestamp.desc())
        .load(&mut conn)
        .await
        .context("failed to query transactions")
        .map_err(AppError::from)?;

    Ok(Json(GetTransactionsResponse {
        transactions: transactions
            .into_iter()
            .map(|transaction| TransactionResponse {
                id: transaction.id,
                amount: transaction.amount,
                recipient: transaction.recipient,
                sender: transaction.sender,
                timestamp: transaction.timestamp,
            })
            .collect(),
    }))
}

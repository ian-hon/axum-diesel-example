use anyhow::Context as _;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Result;
use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::AsyncConnection as _;
#[allow(
    clippy::unused_trait_names,
    reason = "error[E0034]: multiple applicable items in scope"
)]
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

use crate::error::{AppError, JsonRejection};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::transaction::NewTransaction;
use crate::models::{Transaction, User};
use crate::state::DbConnectionPool;

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
    State(pool): State<DbConnectionPool>,
    Extension(authenticated_user): Extension<AuthenticatedUser>,
    WithRejection(Json(payload), _): WithRejection<Json<PostTranscactionPayload>, JsonRejection>,
) -> Result<Json<PostTransactionResponse>> {
    use crate::models::types;
    use crate::schema::{transactions, users};

    let mut conn = pool
        .get()
        .await
        .context("failed to get database connection")
        .map_err(AppError::from)?;

    if authenticated_user.subject != payload.sender {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "title": "PermissionDenied",
            })),
        ))?;
    }

    let created_transaction = conn
        .transaction(|conn| {
            Box::pin(async move {
                let mut sender: User = users::table
                    .find(types::Uuid::from(payload.sender))
                    .select(User::as_select())
                    .first(conn)
                    .await
                    .context("could not find user")?;

                let mut recipient: User = match users::table
                    .find(types::Uuid::from(payload.recipient))
                    .select(User::as_select())
                    .first(conn)
                    .await
                {
                    Ok(recipient) => recipient,
                    Err(diesel::NotFound) => {
                        debug!(%payload.recipient, "could not find recipient");

                        return Ok(Err((
                            StatusCode::BAD_REQUEST,
                            Json(json!({
                                "title": "InvalidRecipient",
                            })),
                        )));
                    },
                    Err(err) => {
                        return Err(err).context("failed to query users")?;
                    },
                };

                if sender.balance < payload.amount {
                    return Ok(Err((
                        StatusCode::FORBIDDEN,
                        Json(json!({
                            "title": "InsufficientBalance",
                        })),
                    )));
                }

                let new_transaction = NewTransaction {
                    id: Uuid::now_v7(),
                    amount: payload.amount,
                    recipient: recipient.id,
                    sender: sender.id,
                    timestamp: jiff::Timestamp::now(),
                };

                let created_transaction: Transaction = diesel::insert_into(transactions::table)
                    .values(new_transaction)
                    .returning(Transaction::as_returning())
                    .get_result(conn)
                    .await
                    .context("failed to insert transaction")?;

                sender.balance -= &created_transaction.amount;
                recipient.balance += &created_transaction.amount;

                let _sender: User = diesel::update(users::table.find(types::Uuid::from(sender.id)))
                    .set(sender)
                    .returning(User::as_returning())
                    .get_result(conn)
                    .await
                    .context("failed to update user")?;

                let _recipient: User =
                    diesel::update(users::table.find(types::Uuid::from(recipient.id)))
                        .set(recipient)
                        .returning(User::as_returning())
                        .get_result(conn)
                        .await
                        .context("failed to update user")?;

                Ok::<_, anyhow::Error>(Ok(created_transaction))
            })
        })
        .await
        .map_err(AppError::from)??;

    Ok(Json(PostTransactionResponse {
        id: created_transaction.id,
        amount: created_transaction.amount,
        recipient: created_transaction.recipient,
        sender: created_transaction.sender,
        timestamp: created_transaction.timestamp,
    }))
}

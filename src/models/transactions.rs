use serde::Deserialize;
use serde::Serialize;

use crate::models::user::UserError;
use crate::models::user::fetch_mut;
use crate::state::AppState;
use crate::utils::fetch_unix_time;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub amount: f64,
    pub target: String,
    pub origin: String,
    pub timestamp: u128, // epoch unix
}

pub async fn fetch_all(state: &AppState, username: &String) -> Vec<Transaction> {
    // STUB
    // fetch all transactions whos transaction.origin == username
    vec![]
}

pub async fn fetch(state: &AppState, id: u64) -> Option<Transaction> {
    // STUB
    None
}

async fn create(state: &AppState, amount: f64, target: &String, origin: &String) -> Transaction {
    // TODO: another check for target and origin existance?
    // STUB
    Transaction {
        id: 0,
        amount,
        target: target.clone(),
        origin: origin.clone(),
        timestamp: fetch_unix_time(),
    }
}

pub async fn transfer_amount(
    state: &AppState,
    amount: f64,
    target: &String,
    origin: &String,
) -> Result<Transaction, TransactionError> {
    if let (Some(from), Some(to)) = (
        fetch_mut(state, origin).await,
        fetch_mut(state, target).await,
    ) {
        if amount > from.balance {
            return Err(TransactionError::InsufficientBalance);
        }

        from.balance -= amount;
        to.balance += amount;

        // TODO: announce outgoing and incoming transaction (websockets)

        return Ok(create(state, amount, target, origin).await);
    }

    Err(TransactionError::User(UserError::NoExist))
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionError {
    User(UserError),

    InsufficientBalance,
}

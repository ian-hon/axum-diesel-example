use serde::Deserialize;
use serde::Serialize;

use crate::models::auth_body::AuthBody;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String, // treat as unique identifier
    pub password_hash: String,
    pub balance: f64,
}

pub async fn fetch(state: &AppState, username: &String) -> Option<User> {
    // STUB
    None
}

// TODO: refactor needed, unsure about diesel structure
pub async fn fetch_mut<'a>(state: &AppState, username: &String) -> Option<&'a mut User> {
    // STUB
    None
}

// For simplicity sake, just use auth to create as well
pub async fn create(state: &AppState, auth: AuthBody) -> Result<User, UserError> {
    // STUB
    Err(UserError::Exists)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserError {
    NoExist,
    Exists,

    PasswordIncorrect,
}

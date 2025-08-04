use serde::Deserialize;
use serde::Serialize;

use crate::models::user::User;
use crate::models::user::UserError;
use crate::models::user::fetch;
use crate::state::AppState;

// This will be passed in the body of post requests,
// and will function as the authentication system
// for requests.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    pub username: String,
    pub password_hash: String,
}
impl AuthBody {
    pub async fn validate(&self, state: &AppState) -> Result<User, UserError> {
        if let Some(u) = fetch(state, &self.username).await {
            if u.password_hash == self.password_hash {
                return Ok(u);
            }
            return Err(UserError::PasswordIncorrect);
        }

        Err(UserError::NoExist)
    }
}

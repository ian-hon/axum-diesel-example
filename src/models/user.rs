use bigdecimal::BigDecimal;
use secrecy::SecretString;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: SecretString,
    pub balance: BigDecimal,
}

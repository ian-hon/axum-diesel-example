use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use secrecy::SecretString;
use uuid::Uuid;

use super::types;
use crate::schema::users;

#[derive(Debug, AsChangeset, Identifiable, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Sqlite))]
pub struct User {
    #[diesel(
        serialize_as = types::Uuid,
        deserialize_as = types::Uuid,
    )]
    pub id: Uuid,
    pub username: String,
    #[diesel(
        serialize_as = types::SecretString,
        deserialize_as = types::SecretString,
    )]
    pub password_hash: SecretString,
    #[diesel(
        serialize_as = types::BigDecimal,
        deserialize_as = types::BigDecimal,
    )]
    pub balance: BigDecimal,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[diesel(serialize_as = types::Uuid)]
    pub id: Uuid,
    pub username: String,
    #[diesel(serialize_as = types::SecretString)]
    pub password_hash: SecretString,
    #[diesel(serialize_as = types::BigDecimal)]
    pub balance: BigDecimal,
}

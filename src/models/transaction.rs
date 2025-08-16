use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use uuid::Uuid;

use super::types;
use crate::schema::transactions;

#[derive(Debug, AsChangeset, Identifiable, Queryable, Selectable)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(Sqlite))]
pub struct Transaction {
    #[diesel(
        serialize_as = types::Uuid,
        deserialize_as = types::Uuid,
    )]
    pub id: Uuid,
    #[diesel(
        serialize_as = types::BigDecimal,
        deserialize_as = types::BigDecimal,
    )]
    pub amount: BigDecimal,
    #[diesel(
        serialize_as = types::Uuid,
        deserialize_as = types::Uuid,
    )]
    pub recipient: Uuid,
    #[diesel(
        serialize_as = types::Uuid,
        deserialize_as = types::Uuid,
    )]
    pub sender: Uuid,
    #[diesel(
        serialize_as = jiff_diesel::Timestamp,
        deserialize_as = jiff_diesel::Timestamp,
    )]
    pub timestamp: jiff::Timestamp,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    #[diesel(serialize_as = types::Uuid)]
    pub id: Uuid,
    #[diesel(serialize_as = types::BigDecimal)]
    pub amount: BigDecimal,
    #[diesel(serialize_as = types::Uuid)]
    pub recipient: Uuid,
    #[diesel(serialize_as = types::Uuid)]
    pub sender: Uuid,
    #[diesel(serialize_as = jiff_diesel::Timestamp)]
    pub timestamp: jiff::Timestamp,
}

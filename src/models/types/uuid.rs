use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Binary;
use diesel::sqlite::{Sqlite, SqliteValue};
use diesel::{AsExpression, FromSqlRow};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, AsExpression, FromSqlRow)]
#[diesel(sql_type = Binary)]
pub struct Uuid(uuid::Uuid);

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(value: Uuid) -> Self {
        value.0
    }
}

impl FromSql<Binary, Sqlite> for Uuid {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        let bytes = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        let value = uuid::Uuid::from_slice(&bytes)?;

        Ok(Uuid(value))
    }
}

impl ToSql<Binary, Sqlite> for Uuid {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = &self.0.as_bytes()[..];

        <[u8] as ToSql<Binary, Sqlite>>::to_sql(value, out)
    }
}

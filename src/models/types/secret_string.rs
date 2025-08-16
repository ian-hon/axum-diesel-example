use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::{Sqlite, SqliteValue};
use diesel::{AsExpression, FromSqlRow};
use secrecy::ExposeSecret as _;

#[derive(Clone, Debug, Default, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct SecretString(secrecy::SecretString);

impl From<secrecy::SecretString> for SecretString {
    fn from(value: secrecy::SecretString) -> Self {
        Self(value)
    }
}

impl From<SecretString> for secrecy::SecretString {
    fn from(value: SecretString) -> Self {
        value.0
    }
}

impl FromSql<Text, Sqlite> for SecretString {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let value = secrecy::SecretString::from(s);

        Ok(SecretString(value))
    }
}

impl ToSql<Text, Sqlite> for SecretString {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = self.0.expose_secret();

        <str as ToSql<Text, Sqlite>>::to_sql(value, out)
    }
}

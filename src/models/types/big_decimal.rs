use std::str::FromStr as _;

use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::{Sqlite, SqliteValue};
use diesel::{AsExpression, FromSqlRow};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct BigDecimal(bigdecimal::BigDecimal);

impl From<bigdecimal::BigDecimal> for BigDecimal {
    fn from(value: bigdecimal::BigDecimal) -> Self {
        Self(value)
    }
}

impl From<BigDecimal> for bigdecimal::BigDecimal {
    fn from(value: BigDecimal) -> Self {
        value.0
    }
}

impl FromSql<Text, Sqlite> for BigDecimal {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let value = bigdecimal::BigDecimal::from_str(&s)?;

        Ok(BigDecimal(value))
    }
}

impl ToSql<Text, Sqlite> for BigDecimal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = &self.0;

        out.set_value(format!("{value}"));
        Ok(IsNull::No)
    }
}

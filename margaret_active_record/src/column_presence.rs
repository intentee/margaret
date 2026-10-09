use std::error::Error;

use tokio_postgres::types::FromSql;
use tokio_postgres::types::Type;

pub(crate) enum ColumnPresence {
    Null,
    Present,
}

impl<'row> FromSql<'row> for ColumnPresence {
    fn from_sql(_: &Type, _: &'row [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(Self::Present)
    }

    fn from_sql_null(_: &Type) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(Self::Null)
    }

    fn accepts(_: &Type) -> bool {
        true
    }
}

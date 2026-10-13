use std::error::Error;

use bytes::BytesMut;
use tokio_postgres::types::IsNull;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::types::to_sql_checked;

#[derive(Debug)]
pub(crate) struct NullParameter;

impl ToSql for NullParameter {
    fn to_sql(&self, _: &Type, _: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        Ok(IsNull::Yes)
    }

    fn accepts(_: &Type) -> bool {
        true
    }

    to_sql_checked!();
}

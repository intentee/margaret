use std::error::Error;

use bytes::BytesMut;
use tokio_postgres::types::FromSql;
use tokio_postgres::types::IsNull;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::types::to_sql_checked;

#[derive(Clone, Debug)]
pub struct RawColumn {
    encoded: Vec<u8>,
}

impl<'row> FromSql<'row> for RawColumn {
    fn from_sql(_: &Type, raw: &'row [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(Self {
            encoded: raw.to_vec(),
        })
    }

    fn accepts(_: &Type) -> bool {
        true
    }
}

impl ToSql for RawColumn {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        out.extend_from_slice(&self.encoded);

        Ok(IsNull::No)
    }

    fn accepts(_: &Type) -> bool {
        true
    }

    to_sql_checked!();
}

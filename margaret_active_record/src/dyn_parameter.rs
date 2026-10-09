use std::error::Error;

use bytes::BytesMut;
use tokio_postgres::types::IsNull;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::types::to_sql_checked;

use margaret_sql::sql_parameter::SqlParameter;

#[derive(Debug)]
pub struct DynParameter {
    pub(crate) parameter: SqlParameter,
}

impl ToSql for DynParameter {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.parameter.value.to_sql_checked(ty, out)
    }

    fn accepts(_: &Type) -> bool {
        true
    }

    to_sql_checked!();
}

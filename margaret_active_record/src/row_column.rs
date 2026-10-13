use tokio_postgres::Row;
use tokio_postgres::types::FromSqlOwned;

use margaret_model::table::Table;

use crate::active_record_error::ActiveRecordError;

pub(crate) fn row_column<Read: FromSqlOwned>(
    row: &Row,
    position: usize,
    table: &'static Table,
) -> Result<Read, ActiveRecordError> {
    row.try_get(position)
        .map_err(|source| ActiveRecordError::MalformedColumn {
            position,
            source,
            table: table.name,
        })
}

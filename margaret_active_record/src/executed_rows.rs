use tokio_postgres::Row;

use margaret_database::executor::Executor;
use margaret_model::table::Table;
use margaret_sql::statement::Statement;

use crate::active_record_error::ActiveRecordError;
use crate::statement_error::statement_error;
use crate::statement_kind::StatementKind;

pub(crate) async fn executed_rows<Executing: Executor>(
    statement: Result<Statement, ActiveRecordError>,
    kind: StatementKind,
    table: &'static Table,
    executor: &Executing,
) -> Result<Vec<Row>, ActiveRecordError> {
    let statement = statement?;

    executor
        .rows(&statement)
        .await
        .map_err(statement_error(kind, table.name))
}

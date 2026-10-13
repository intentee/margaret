use deadpool_postgres::GenericClient;
use tokio_postgres::Row;

use margaret_sql::statement::Statement;

use crate::database_error::DatabaseError;
use crate::statement_parameters::statement_parameters;

pub(crate) async fn statement_optional_row(
    client: &impl GenericClient,
    statement: &Statement,
) -> Result<Option<Row>, DatabaseError> {
    let prepared = client
        .prepare_cached(&statement.text)
        .await
        .map_err(DatabaseError::StatementPreparation)?;

    client
        .query_opt(&prepared, &statement_parameters(statement))
        .await
        .map_err(DatabaseError::StatementExecution)
}

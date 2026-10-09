use deadpool_postgres::GenericClient;

use margaret_sql::statement::Statement;

use crate::database_error::DatabaseError;
use crate::statement_parameters::statement_parameters;

pub(crate) async fn statement_affected(
    client: &impl GenericClient,
    statement: &Statement,
) -> Result<u64, DatabaseError> {
    let prepared = client
        .prepare_cached(&statement.text)
        .await
        .map_err(DatabaseError::StatementPreparation)?;

    client
        .execute(&prepared, &statement_parameters(statement))
        .await
        .map_err(DatabaseError::StatementExecution)
}

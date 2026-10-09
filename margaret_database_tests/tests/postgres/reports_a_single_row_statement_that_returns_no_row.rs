use margaret_database::database_error::DatabaseError;
use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;

use crate::postgres::create_probes::create_probes;
use crate::postgres::select_probe_amounts::select_probe_amounts;

#[tokio::test]
async fn reports_a_single_row_statement_that_returns_no_row() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;

    assert!(matches!(
        started.database.row(&select_probe_amounts()).await,
        Err(DatabaseError::StatementExecution(source)) if source.as_db_error().is_none()
    ));
}

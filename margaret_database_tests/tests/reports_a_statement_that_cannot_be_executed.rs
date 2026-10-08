use tokio_postgres::error::SqlState;

use margaret_database::database_error::DatabaseError;
use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;

use crate::create_probes::create_probes;
use crate::insert_probe::insert_probe;

#[tokio::test]
async fn reports_a_statement_that_cannot_be_executed() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;
    started
        .database
        .affected(&insert_probe(1, 10, ConflictAction::Raise))
        .await
        .expect("the first probe is inserted");

    let Err(DatabaseError::StatementExecution(source)) = started
        .database
        .rows(&insert_probe(1, 20, ConflictAction::Raise))
        .await
    else {
        panic!("a probe with a taken key is not inserted");
    };

    assert_eq!(source.code(), Some(&SqlState::UNIQUE_VIOLATION));
}

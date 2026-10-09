use tokio_postgres::error::SqlState;

use margaret_database::database_error::DatabaseError;
use margaret_database::executor::Executor;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;

use crate::postgres::create_probes::create_probes;
use crate::postgres::insert_probe::insert_probe;

#[tokio::test]
async fn reports_a_transaction_that_cannot_commit() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;
    started
        .execute(
            "ALTER TABLE probes ADD CONSTRAINT probes_amount_key UNIQUE (amount) DEFERRABLE INITIALLY DEFERRED",
        )
        .await;

    let mut connection = started
        .database
        .connection()
        .await
        .expect("a connection is checked out");
    let transaction = connection
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the transaction begins");

    for id in [1, 2] {
        transaction
            .affected(&insert_probe(id, 10, ConflictAction::Raise))
            .await
            .expect("the deferred constraint admits the probe until the commit");
    }

    let Err(DatabaseError::Commit(source)) = transaction.commit().await else {
        panic!("the deferred constraint rejects the commit");
    };

    assert_eq!(source.code(), Some(&SqlState::UNIQUE_VIOLATION));
}

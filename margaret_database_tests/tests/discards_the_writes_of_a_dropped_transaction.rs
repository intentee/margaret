use margaret_database::executor::Executor;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;

use crate::create_probes::create_probes;
use crate::insert_probe::insert_probe;
use crate::probe_amounts::probe_amounts;

#[tokio::test]
async fn discards_the_writes_of_a_dropped_transaction() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;

    let mut connection = started
        .database
        .connection()
        .await
        .expect("a connection is checked out");

    {
        let transaction = connection
            .transaction(Isolation::ReadCommitted)
            .await
            .expect("the transaction begins");

        transaction
            .affected(&insert_probe(1, 10, ConflictAction::Raise))
            .await
            .expect("the probe is inserted");
    }

    let transaction = connection
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the connection begins another transaction");

    assert!(probe_amounts(&transaction).await.is_empty());
    assert!(probe_amounts(started.database.as_ref()).await.is_empty());
}

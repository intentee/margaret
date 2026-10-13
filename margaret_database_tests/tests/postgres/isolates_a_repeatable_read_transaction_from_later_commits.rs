use margaret_database::executor::Executor;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;

use crate::postgres::create_probes::create_probes;
use crate::postgres::insert_probe::insert_probe;
use crate::postgres::probe_amounts::probe_amounts;

#[tokio::test]
async fn isolates_a_repeatable_read_transaction_from_later_commits() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;

    let mut connection = started
        .database
        .connection()
        .await
        .expect("a connection is checked out");
    let transaction = connection
        .transaction(Isolation::RepeatableRead)
        .await
        .expect("the transaction begins");

    assert!(probe_amounts(&transaction).await.is_empty());

    started
        .database
        .affected(&insert_probe(1, 10, ConflictAction::Raise))
        .await
        .expect("another connection inserts a probe");

    assert!(probe_amounts(&transaction).await.is_empty());
    assert_eq!(probe_amounts(started.database.as_ref()).await, vec![10]);
}

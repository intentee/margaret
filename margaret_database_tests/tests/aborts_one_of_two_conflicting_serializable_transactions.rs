use tokio_postgres::error::SqlState;

use margaret_database::database_error::DatabaseError;
use margaret_database::executor::Executor;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;

use crate::create_probes::create_probes;
use crate::insert_probe::insert_probe;
use crate::probe_amounts::probe_amounts;

#[tokio::test]
async fn aborts_one_of_two_conflicting_serializable_transactions() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;

    let mut first_connection = started
        .database
        .connection()
        .await
        .expect("a connection is checked out");
    let mut second_connection = started
        .database
        .connection()
        .await
        .expect("another connection is checked out");
    let first = first_connection
        .transaction(Isolation::Serializable)
        .await
        .expect("the first transaction begins");
    let second = second_connection
        .transaction(Isolation::Serializable)
        .await
        .expect("the second transaction begins");

    assert!(probe_amounts(&first).await.is_empty());
    assert!(probe_amounts(&second).await.is_empty());

    first
        .affected(&insert_probe(1, 10, ConflictAction::Raise))
        .await
        .expect("the first transaction inserts a probe");
    second
        .affected(&insert_probe(2, 20, ConflictAction::Raise))
        .await
        .expect("the second transaction inserts a probe");
    first.commit().await.expect("the first transaction commits");

    let Err(DatabaseError::Commit(source)) = second.commit().await else {
        panic!("the second transaction fails to serialize");
    };

    assert_eq!(source.code(), Some(&SqlState::T_R_SERIALIZATION_FAILURE));
}

use margaret_database::database_error::DatabaseError;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_transaction_that_cannot_roll_back() {
    let started = StartedDatabase::start().await;
    let mut connection = started
        .database
        .connection()
        .await
        .expect("a connection is checked out");
    let transaction = connection
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the transaction begins");

    started.administration.make_unreachable().await;

    assert!(matches!(
        transaction.rollback().await,
        Err(DatabaseError::Rollback(_))
    ));
}

use margaret_database::database_error::DatabaseError;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_transaction_that_cannot_begin_with_its_isolation_to_aid_diagnosis() {
    let started = StartedDatabase::start().await;
    let mut connection = started
        .database
        .connection()
        .await
        .expect("a connection is checked out");

    started.administration.make_unreachable().await;

    assert!(matches!(
        connection.transaction(Isolation::ReadCommitted).await,
        Err(DatabaseError::Begin {
            isolation: Isolation::ReadCommitted,
            ..
        })
    ));
}

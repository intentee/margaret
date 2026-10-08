use margaret_database::executor::Executor;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;

use crate::create_probes::create_probes;
use crate::select_probe_amounts::select_probe_amounts;

#[tokio::test]
async fn reads_an_optional_row_inside_a_transaction() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;

    let mut connection = started
        .database
        .connection()
        .await
        .expect("a connection is checked out");
    let transaction = connection
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the transaction begins");

    assert!(
        transaction
            .optional_row(&select_probe_amounts())
            .await
            .expect("the absent probe is read")
            .is_none()
    );
}

use futures_util::join;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::database::isolation::Isolation;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_serialization_failure() {
    let started = started_with_models().await;

    inserted_counter(&started.database, "visits", 1).await;

    let mut first_connection = started
        .database
        .connection()
        .await
        .expect("the first connection is checked out");
    let mut second_connection = started
        .database
        .connection()
        .await
        .expect("the second connection is checked out");
    let first = first_connection
        .transaction(Isolation::RepeatableRead)
        .await
        .expect("the first transaction begins");
    let second = second_connection
        .transaction(Isolation::RepeatableRead)
        .await
        .expect("the second transaction begins");

    Counter::query()
        .name
        .eq("visits".to_string())
        .update(&first, |columns| columns.hits.to(2))
        .await
        .expect("the first transaction updates the counter");

    let (conflicting, committed) = join!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .update(&second, |columns| columns.hits.to(3)),
        async {
            started.administration.await_lock_waiters(1).await;
            first.commit().await
        }
    );

    committed.expect("the first transaction commits");

    assert!(matches!(
        conflicting,
        Err(ActiveRecordError::SerializationFailure {
            statement: StatementKind::Update,
            table: "counters",
            ..
        })
    ));
}

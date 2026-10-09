use futures_util::join;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::database::isolation::Isolation;
use margaret_active_record_tests::models::counter::Counter;

use crate::postgres::inserted_counter::inserted_counter;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_deadlock() {
    let started = started_with_models().await;

    inserted_counter(&started.database, "first", 1).await;
    inserted_counter(&started.database, "second", 1).await;

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
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the first transaction begins");
    let second = second_connection
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the second transaction begins");

    Counter::query()
        .name
        .eq("first".to_string())
        .update(&first, |columns| columns.hits.to(2))
        .await
        .expect("the first transaction locks the first counter");
    Counter::query()
        .name
        .eq("second".to_string())
        .update(&second, |columns| columns.hits.to(2))
        .await
        .expect("the second transaction locks the second counter");

    let (crossing, deadlocked) = join!(
        Counter::query()
            .name
            .eq("second".to_string())
            .update(&first, |columns| columns.hits.to(3)),
        async {
            started.administration.await_lock_waiters(1).await;
            Counter::query()
                .name
                .eq("first".to_string())
                .update(&second, |columns| columns.hits.to(3))
                .await
        }
    );

    assert!([&crossing, &deadlocked].iter().any(|outcome| matches!(
        outcome,
        Err(ActiveRecordError::Deadlock {
            statement: StatementKind::Update,
            table: "counters",
            ..
        })
    )));
    assert!(
        [crossing, deadlocked]
            .into_iter()
            .any(|outcome| matches!(outcome, Ok(Change::Changed(_))))
    );
}

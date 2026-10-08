use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_duplicate_insert_as_a_unique_violation() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    inserted_counter(database, "visits", 1).await;

    assert!(matches!(
        Counter {
            name: "visits".to_string(),
            hits: 2,
        }
        .insert()
        .run(database)
        .await,
        Err(ActiveRecordError::UniqueViolation {
            statement: StatementKind::Insert,
            table: "counters",
            ..
        })
    ));
}

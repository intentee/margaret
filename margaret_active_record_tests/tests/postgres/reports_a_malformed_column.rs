use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::counter::Counter;

use crate::postgres::inserted_counter::inserted_counter;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_malformed_column() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    inserted_counter(database, "visits", 1).await;
    started
        .administration
        .execute("ALTER TABLE counters ALTER COLUMN hits TYPE TEXT")
        .await;

    assert!(matches!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .find(database)
            .await,
        Err(ActiveRecordError::MalformedColumn {
            position: 1,
            table: "counters",
            ..
        })
    ));
}

use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::upsertion::Upsertion;
use margaret_active_record_tests::models::counter::Counter;

use crate::postgres::inserted_counter::inserted_counter;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn keeps_a_row_whose_upsert_condition_fails() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    inserted_counter(database, "visits", 7).await;

    assert_eq!(
        Counter {
            name: "visits".to_string(),
            hits: 3,
        }
        .insert()
        .or_update(|columns| columns.hits.excluded())
        .when(|existing| existing.hits.at_most(5))
        .run(database)
        .await
        .expect("the upsert completes"),
        Upsertion::Kept
    );
}

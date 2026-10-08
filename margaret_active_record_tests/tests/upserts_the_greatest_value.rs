use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn upserts_the_greatest_value() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let kept = inserted_counter(database, "visits", 7).await;

    Counter {
        name: "visits".to_string(),
        hits: 3,
    }
    .insert()
    .or_update(|columns| columns.hits.greatest())
    .run(database)
    .await
    .expect("the counter is upserted");

    assert_eq!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .find(database)
            .await
            .expect("the counter is read"),
        Lookup::Found(kept)
    );
}

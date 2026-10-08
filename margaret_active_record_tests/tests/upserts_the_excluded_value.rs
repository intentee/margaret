use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::upsertion::Upsertion;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn upserts_the_excluded_value() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let replacement = Counter {
        name: "visits".to_string(),
        hits: 3,
    };

    inserted_counter(database, "visits", 7).await;

    assert_eq!(
        replacement
            .insert()
            .or_update(|columns| columns.hits.excluded())
            .run(database)
            .await
            .expect("the counter is upserted"),
        Upsertion::Written
    );
    assert_eq!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .find(database)
            .await
            .expect("the counter is read"),
        Lookup::Found(replacement)
    );
}

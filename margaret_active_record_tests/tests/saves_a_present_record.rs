use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::saving::Saving;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn saves_a_present_record() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let mut counter = inserted_counter(database, "visits", 1).await;

    counter.hits = 2;

    assert_eq!(
        counter.save(database).await.expect("the counter is saved"),
        Saving::Saved
    );
    assert_eq!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .find(database)
            .await
            .expect("the counter is read"),
        Lookup::Found(counter)
    );
}

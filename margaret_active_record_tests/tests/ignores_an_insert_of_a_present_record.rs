use margaret::framework::active_record::insertion::Insertion;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn ignores_an_insert_of_a_present_record() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let present = inserted_counter(database, "visits", 1).await;

    assert_eq!(
        Counter {
            name: "visits".to_string(),
            hits: 9,
        }
        .insert()
        .or_ignore(database)
        .await
        .expect("the conflicting insert is ignored"),
        Insertion::AlreadyPresent
    );
    assert_eq!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .find(database)
            .await
            .expect("the counter is read"),
        Lookup::Found(present)
    );
}

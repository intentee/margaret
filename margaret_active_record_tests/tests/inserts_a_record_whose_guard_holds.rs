use margaret::framework::active_record::guarded_insertion::GuardedInsertion;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn inserts_a_record_whose_guard_holds() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    inserted_counter(database, "visits", 1).await;

    assert_eq!(
        Counter {
            name: "downloads".to_string(),
            hits: 1,
        }
        .insert()
        .when(Counter::query().name.eq("visits".to_string()).exists())
        .run(database)
        .await
        .expect("the guarded insert completes"),
        GuardedInsertion::Inserted
    );
}

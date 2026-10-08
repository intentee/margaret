use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::removal::Removal;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn deletes_a_unique_row() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let counter = inserted_counter(database, "visits", 1).await;

    assert_eq!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .delete(database)
            .await
            .expect("the counter is deleted"),
        Removal::Removed(counter)
    );
}

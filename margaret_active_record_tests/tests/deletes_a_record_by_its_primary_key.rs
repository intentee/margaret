use margaret::framework::active_record::deletion::Deletion;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::counter::Counter;

use crate::inserted_counter::inserted_counter;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn deletes_a_record_by_its_primary_key() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let counter = inserted_counter(database, "visits", 1).await;

    assert_eq!(
        counter
            .delete(database)
            .await
            .expect("the record is deleted"),
        Deletion::Deleted
    );
    assert_eq!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .find(database)
            .await
            .expect("the counter is looked up"),
        Lookup::Missing
    );
}

use margaret::framework::active_record::insertion::Insertion;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::counter::Counter;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn inserts_a_record_that_is_not_present() {
    let started = started_with_models().await;

    assert_eq!(
        Counter {
            name: "visits".to_string(),
            hits: 1,
        }
        .insert()
        .or_ignore(started.database.as_ref())
        .await
        .expect("the counter is inserted"),
        Insertion::Inserted
    );
}

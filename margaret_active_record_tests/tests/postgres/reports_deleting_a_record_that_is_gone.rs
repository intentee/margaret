use margaret::framework::active_record::deletion::Deletion;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::counter::Counter;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn reports_deleting_a_record_that_is_gone() {
    let started = started_with_models().await;

    assert_eq!(
        Counter {
            name: "visits".to_string(),
            hits: 1,
        }
        .delete(started.database.as_ref())
        .await
        .expect("the delete completes"),
        Deletion::Missing
    );
}

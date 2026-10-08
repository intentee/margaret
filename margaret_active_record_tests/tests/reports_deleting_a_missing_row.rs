use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::removal::Removal;
use margaret_active_record_tests::models::counter::Counter;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_deleting_a_missing_row() {
    let started = started_with_models().await;

    assert_eq!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .delete(started.database.as_ref())
            .await
            .expect("the delete completes"),
        Removal::Missing
    );
}

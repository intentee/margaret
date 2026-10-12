use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::saving::Saving;
use margaret_active_record_tests::models::counter::Counter;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn reports_saving_a_missing_record_as_missing_so_callers_know_nothing_was_saved() {
    let started = started_with_models().await;

    assert_eq!(
        Counter {
            name: "visits".to_string(),
            hits: 2,
        }
        .save(started.database.as_ref())
        .await
        .expect("the save completes"),
        Saving::Missing
    );
}

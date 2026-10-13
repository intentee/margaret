use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::key_set::KeySet;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn leaves_a_unique_row_whose_condition_fails_unchanged() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    KeySet {
        name: "jwks".to_string(),
        generation: 2,
        document: "{}".to_string(),
    }
    .insert()
    .run(database)
    .await
    .expect("the key set is inserted");

    assert_eq!(
        KeySet::query()
            .name
            .eq("jwks".to_string())
            .when(|keys| keys.generation.eq(1))
            .update(database, |columns| columns.generation.to(3))
            .await
            .expect("the update completes"),
        Change::Unmatched
    );
}

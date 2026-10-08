use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::key_set::KeySet;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn updates_a_unique_row() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    KeySet {
        name: "jwks".to_string(),
        generation: 1,
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
            .update(database, |columns| {
                columns
                    .generation
                    .to(2)
                    .and(columns.document.to("[]".to_string()))
            })
            .await
            .expect("the key set is updated"),
        Change::Changed(KeySet {
            name: "jwks".to_string(),
            generation: 2,
            document: "[]".to_string(),
        })
    );
}

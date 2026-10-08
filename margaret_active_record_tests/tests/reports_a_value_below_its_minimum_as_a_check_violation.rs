use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret_active_record_tests::models::key_set::KeySet;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_value_below_its_minimum_as_a_check_violation() {
    let started = started_with_models().await;

    assert!(matches!(
        KeySet {
            name: "signing".to_string(),
            generation: -1,
            document: "{}".to_string(),
        }
        .insert()
        .run(started.database.as_ref())
        .await,
        Err(ActiveRecordError::CheckViolation {
            statement: StatementKind::Insert,
            table: "key_sets",
            ..
        })
    ));
}

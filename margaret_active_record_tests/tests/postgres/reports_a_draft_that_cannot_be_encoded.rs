use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::creatable::Creatable;
use margaret_active_record_tests::margaret::models::models_payload_payload::draft::Draft;
use margaret_active_record_tests::models::payload::Payload;

use crate::postgres::started_with_models::started_with_models;
use crate::postgres::unserializable_document::unserializable_document;

#[tokio::test]
async fn reports_a_draft_that_cannot_be_encoded() {
    let started = started_with_models().await;

    assert!(matches!(
        Payload::create(Draft {
            document: unserializable_document(),
        })
        .run(started.database.as_ref())
        .await,
        Err(ActiveRecordError::JsonSerialization {
            position: 0,
            table: "payloads",
            ..
        })
    ));
}

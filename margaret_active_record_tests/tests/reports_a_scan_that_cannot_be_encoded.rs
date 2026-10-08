use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::payload::Payload;

use crate::started_with_models::started_with_models;
use crate::unserializable_document::unserializable_document;

#[tokio::test]
async fn reports_a_scan_that_cannot_be_encoded() {
    let started = started_with_models().await;

    assert!(matches!(
        Payload::query()
            .document
            .eq(unserializable_document())
            .id
            .ascending()
            .limit::<1>()
            .fetch(started.database.as_ref())
            .await,
        Err(ActiveRecordError::JsonSerialization {
            table: "payloads",
            ..
        })
    ));
}

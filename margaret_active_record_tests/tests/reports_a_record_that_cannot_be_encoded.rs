use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::payload::Payload;

use crate::started_with_models::started_with_models;
use crate::unserializable_document::unserializable_document;

#[tokio::test]
async fn reports_a_record_that_cannot_be_encoded() {
    let started = started_with_models().await;

    assert!(matches!(
        Payload {
            id: Uuid::nil(),
            document: unserializable_document(),
        }
        .insert()
        .run(started.database.as_ref())
        .await,
        Err(ActiveRecordError::JsonSerialization {
            position: 1,
            table: "payloads",
            ..
        })
    ));
}

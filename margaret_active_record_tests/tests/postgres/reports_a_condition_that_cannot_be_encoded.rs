use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::payload::Payload;

use crate::postgres::started_with_models::started_with_models;
use crate::postgres::unserializable_document::unserializable_document;

#[tokio::test]
async fn reports_a_condition_that_cannot_be_encoded() {
    let started = started_with_models().await;

    assert!(matches!(
        Payload::query()
            .id
            .eq(Uuid::nil())
            .when(|payload| payload.document.eq(unserializable_document()))
            .find(started.database.as_ref())
            .await,
        Err(ActiveRecordError::JsonSerialization {
            table: "payloads",
            ..
        })
    ));
}

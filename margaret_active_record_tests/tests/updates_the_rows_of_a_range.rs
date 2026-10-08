use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::note::Note;

use crate::at_epoch_seconds::at_epoch_seconds;
use crate::created_note::created_note;
use crate::note_bodies::note_bodies;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn updates_the_rows_of_a_range() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    created_note(database, "expired", Some(at_epoch_seconds(10))).await;
    created_note(database, "current", Some(at_epoch_seconds(30))).await;

    assert_eq!(
        Note::query()
            .expires_at
            .below(at_epoch_seconds(20))
            .update(database, |columns| columns.body.to("archived".to_string()))
            .await
            .expect("the expired notes are updated"),
        1
    );
    assert_eq!(note_bodies(database).await, ["archived", "current"]);
}

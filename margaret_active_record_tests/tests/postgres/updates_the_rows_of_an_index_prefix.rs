use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::note::Note;

use crate::postgres::at_epoch_seconds::at_epoch_seconds;
use crate::postgres::created_note::created_note;
use crate::postgres::note_bodies::note_bodies;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn updates_the_rows_of_an_index_prefix() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    created_note(database, "first", Some(at_epoch_seconds(10))).await;
    created_note(database, "other", Some(at_epoch_seconds(30))).await;

    assert_eq!(
        Note::query()
            .expires_at
            .eq(at_epoch_seconds(10))
            .update(database, |columns| columns.body.to("touched".to_string()))
            .await
            .expect("the notes are updated"),
        1
    );
    assert_eq!(note_bodies(database).await, ["touched", "other"]);
}

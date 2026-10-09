use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::note::Note;

use crate::postgres::at_epoch_seconds::at_epoch_seconds;
use crate::postgres::created_note::created_note;
use crate::postgres::note_bodies::note_bodies;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn deletes_the_rows_of_a_range() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    created_note(database, "expired", Some(at_epoch_seconds(10))).await;
    created_note(database, "current", Some(at_epoch_seconds(30))).await;
    created_note(database, "permanent", None).await;

    assert_eq!(
        Note::query()
            .expires_at
            .at_most(at_epoch_seconds(20))
            .delete(database)
            .await
            .expect("the expired notes are deleted"),
        1
    );
    assert_eq!(note_bodies(database).await, ["current", "permanent"]);
}

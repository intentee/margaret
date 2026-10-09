use margaret::framework::active_record::lookup::Lookup;

use crate::postgres::at_epoch_seconds::at_epoch_seconds;
use crate::postgres::created_note::created_note;
use crate::postgres::note_lookup::note_lookup;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn matches_a_row_at_an_inclusive_lower_bound() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let note = created_note(database, "note", Some(at_epoch_seconds(10))).await;

    assert_eq!(
        note_lookup(database, &note, |row| row
            .expires_at
            .at_least(at_epoch_seconds(10)))
        .await,
        Lookup::Found(note)
    );
}

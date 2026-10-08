use margaret::framework::active_record::lookup::Lookup;

use crate::at_epoch_seconds::at_epoch_seconds;
use crate::created_note::created_note;
use crate::note_lookup::note_lookup;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn skips_a_row_at_an_exclusive_lower_bound() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let note = created_note(database, "note", Some(at_epoch_seconds(10))).await;

    assert_eq!(
        note_lookup(database, &note, |row| row
            .expires_at
            .above(at_epoch_seconds(10)))
        .await,
        Lookup::Missing
    );
}

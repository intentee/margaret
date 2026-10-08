use margaret::framework::active_record::lookup::Lookup;

use crate::at_epoch_seconds::at_epoch_seconds;
use crate::created_note::created_note;
use crate::note_lookup::note_lookup;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn matches_a_row_satisfying_both_conditions() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let note = created_note(database, "note", Some(at_epoch_seconds(10))).await;

    assert_eq!(
        note_lookup(database, &note, |row| {
            row.body
                .eq("note".to_string())
                .and(row.expires_at.below(at_epoch_seconds(20)))
        })
        .await,
        Lookup::Found(note)
    );
}

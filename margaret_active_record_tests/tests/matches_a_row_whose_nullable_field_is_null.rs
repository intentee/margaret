use margaret::framework::active_record::lookup::Lookup;

use crate::created_note::created_note;
use crate::note_lookup::note_lookup;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn matches_a_row_whose_nullable_field_is_null() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let note = created_note(database, "note", None).await;

    assert_eq!(
        note_lookup(database, &note, |row| row.expires_at.is_null()).await,
        Lookup::Found(note)
    );
}

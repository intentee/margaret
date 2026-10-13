use margaret::framework::active_record::lookup::Lookup;

use crate::postgres::created_note::created_note;
use crate::postgres::note_lookup::note_lookup;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn skips_a_row_matching_a_negated_condition() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let note = created_note(database, "note", None).await;

    assert_eq!(
        note_lookup(database, &note, |row| row
            .body
            .eq("note".to_string())
            .negated())
        .await,
        Lookup::Missing
    );
}

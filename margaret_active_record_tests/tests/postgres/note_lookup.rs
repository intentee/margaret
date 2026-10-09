use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::predicate::Predicate;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::margaret::models::models_note_note::conditions::Conditions;
use margaret_active_record_tests::models::note::Note;

pub async fn note_lookup(
    database: &Database,
    note: &Note,
    guard: impl FnOnce(Conditions) -> Predicate<Note>,
) -> Lookup<Note> {
    Note::query()
        .id
        .eq(note.id)
        .when(guard)
        .find(database)
        .await
        .expect("the guarded note is read")
}

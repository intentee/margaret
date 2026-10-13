use chrono::DateTime;
use chrono::Utc;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::margaret::models::models_note_note::draft::Draft;
use margaret_active_record_tests::models::note::Note;

pub async fn created_note(
    database: &Database,
    body: &str,
    expires_at: Option<DateTime<Utc>>,
) -> Note {
    Note::create(Draft {
        body: body.to_string(),
        expires_at,
    })
    .run(database)
    .await
    .expect("the note is created")
}

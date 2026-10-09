use futures_util::TryStreamExt as _;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::note::Note;

pub async fn note_bodies(database: &Database) -> Vec<String> {
    Note::query()
        .id
        .ascending()
        .stream::<10, _>(database)
        .map_ok(|note| note.body)
        .try_collect()
        .await
        .expect("the notes are streamed")
}

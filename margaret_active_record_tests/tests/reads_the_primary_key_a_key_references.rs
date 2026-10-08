use margaret::framework::active_record::key::Key;

use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reads_the_primary_key_a_key_references() {
    let started = started_with_models().await;
    let author = created_author(&started.database, "Milo").await;

    assert_eq!(*Key::of(&author).primary_key(), author.id);
}

use margaret::framework::active_record::key::Key;

use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn describes_a_key_by_its_primary_key() {
    let started = started_with_models().await;
    let author = created_author(&started.database, "Milo").await;

    assert_eq!(
        format!("{:?}", Key::of(&author)),
        format!("Key {{ primary_key: {:?} }}", author.id)
    );
}

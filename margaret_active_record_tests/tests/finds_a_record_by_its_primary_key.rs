use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::author::Author;

use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn finds_a_record_by_its_primary_key() {
    let started = started_with_models().await;
    let author = created_author(&started.database, "Milo").await;

    assert_eq!(
        Author::query()
            .id
            .eq(author.id)
            .find(started.database.as_ref())
            .await
            .expect("the author is read"),
        Lookup::Found(author)
    );
}

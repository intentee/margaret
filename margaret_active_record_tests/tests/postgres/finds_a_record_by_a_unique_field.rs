use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::author::Author;

use crate::postgres::created_author::created_author;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn finds_a_record_by_a_unique_field() {
    let started = started_with_models().await;
    let author = created_author(&started.database, "Milo").await;

    assert_eq!(
        Author::query()
            .name
            .eq("Milo".to_string())
            .find(started.database.as_ref())
            .await
            .expect("the author is read"),
        Lookup::Found(author)
    );
}

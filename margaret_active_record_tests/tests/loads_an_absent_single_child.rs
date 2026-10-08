use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::author::Author;
use margaret_active_record_tests::models::author_with_profile::AuthorWithProfile;

use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn loads_an_absent_single_child() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;

    assert_eq!(
        Author::query()
            .id
            .eq(author.id)
            .load::<AuthorWithProfile, _>(database)
            .await
            .expect("the author is loaded"),
        Lookup::Found(AuthorWithProfile {
            author,
            profile: None,
        })
    );
}

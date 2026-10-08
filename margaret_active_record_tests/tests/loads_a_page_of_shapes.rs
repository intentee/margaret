use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::next_page::NextPage;
use margaret::framework::active_record::page::Page;
use margaret_active_record_tests::models::author::Author;
use margaret_active_record_tests::models::author_with_profile::AuthorWithProfile;

use crate::created_author::created_author;
use crate::profiled::profiled;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn loads_a_page_of_shapes() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let mut authors = [
        created_author(database, "Milo").await,
        created_author(database, "Mona").await,
        created_author(database, "Nina").await,
    ];

    authors.sort_by_key(|author| author.id);

    let profile = profiled(database, &authors[1]).await;
    let Page { next, records } = Author::query()
        .id
        .ascending()
        .limit::<2>()
        .load::<AuthorWithProfile, _>(database)
        .await
        .expect("the page is loaded");

    assert_eq!(
        records,
        vec![
            AuthorWithProfile {
                author: authors[0].clone(),
                profile: None,
            },
            AuthorWithProfile {
                author: authors[1].clone(),
                profile: Some(profile),
            },
        ]
    );
    assert!(matches!(next, NextPage::Continues(_)));
}

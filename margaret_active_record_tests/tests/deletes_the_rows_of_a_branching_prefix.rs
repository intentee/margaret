use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn deletes_the_rows_of_a_branching_prefix() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let milo = created_author(database, "Milo").await;
    let mona = created_author(database, "Mona").await;

    created_article(database, &milo, "First").await;
    created_article(database, &milo, "Second").await;
    created_article(database, &mona, "Kept").await;

    assert_eq!(
        Article::query()
            .author
            .eq(Key::of(&milo))
            .delete(database)
            .await
            .expect("the articles are deleted"),
        2
    );
}

use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;

use crate::postgres::created_article::created_article;
use crate::postgres::created_author::created_author;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn updates_the_rows_of_a_branching_prefix() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let milo = created_author(database, "Milo").await;

    created_article(database, &milo, "First").await;
    created_article(database, &milo, "Second").await;

    assert_eq!(
        Article::query()
            .author
            .eq(Key::of(&milo))
            .update(database, |columns| columns.title.to("Retitled".to_string()))
            .await
            .expect("the articles are updated"),
        2
    );
}

use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn orders_the_rows_of_an_index_prefix() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let milo = created_author(database, "Milo").await;
    let mona = created_author(database, "Mona").await;
    let mut expected = vec![
        created_article(database, &milo, "First").await.id,
        created_article(database, &milo, "Second").await.id,
    ];

    created_article(database, &mona, "Other").await;
    expected.sort();

    assert_eq!(
        Article::query()
            .author
            .eq(Key::of(&milo))
            .id
            .ascending()
            .limit::<5>()
            .fetch(database)
            .await
            .expect("the articles are read")
            .records
            .into_iter()
            .map(|article| article.id)
            .collect::<Vec<_>>(),
        expected
    );
}

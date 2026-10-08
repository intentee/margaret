use margaret::framework::active_record::shape::Shape;
use margaret_active_record_tests::models::article_with_author::ArticleWithAuthor;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn attaches_relations_to_records() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let milo = created_author(database, "Milo").await;
    let mona = created_author(database, "Mona").await;
    let first = created_article(database, &milo, "First").await;
    let second = created_article(database, &mona, "Second").await;

    assert_eq!(
        ArticleWithAuthor::attach(database, &[second.clone(), first.clone()])
            .await
            .expect("the authors are attached"),
        vec![
            ArticleWithAuthor {
                article: second,
                author: mona,
            },
            ArticleWithAuthor {
                article: first,
                author: milo,
            },
        ]
    );
}

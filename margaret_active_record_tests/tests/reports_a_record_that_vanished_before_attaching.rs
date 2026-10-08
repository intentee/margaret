use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::shape::Shape;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_with_author::ArticleWithAuthor;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_record_that_vanished_before_attaching() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let article = created_article(database, &author, "Shipping").await;

    Article::query()
        .id
        .eq(article.id)
        .delete(database)
        .await
        .expect("the article is deleted");

    assert!(matches!(
        ArticleWithAuthor::attach(database, &[article]).await,
        Err(ActiveRecordError::ParentVanished { table: "articles" })
    ));
}

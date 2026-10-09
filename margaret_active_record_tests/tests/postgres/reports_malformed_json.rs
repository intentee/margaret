use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;

use crate::postgres::created_article::created_article;
use crate::postgres::created_author::created_author;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn reports_malformed_json() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let article = created_article(database, &author, "Shipping").await;

    started
        .administration
        .execute("UPDATE articles SET tags = 'not json'")
        .await;

    assert!(matches!(
        Article::query().id.eq(article.id).find(database).await,
        Err(ActiveRecordError::MalformedJson {
            position: 5,
            table: "articles",
            ..
        })
    ));
}

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_an_unknown_enum_variant() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let article = created_article(database, &author, "Shipping").await;

    started
        .administration
        .execute("UPDATE articles SET status = 'Archived'")
        .await;

    assert!(matches!(
        Article::query().id.eq(article.id).find(database).await,
        Err(ActiveRecordError::UnknownVariant {
            enum_type: "crate::models::article_status::ArticleStatus",
            position: 3,
            ref stored,
            table: "articles",
        }) if stored == "Archived"
    ));
}

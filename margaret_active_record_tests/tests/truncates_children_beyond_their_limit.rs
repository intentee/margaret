use margaret::framework::active_record::completeness::Completeness;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_with_translations::ArticleWithTranslations;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;
use crate::translated::translated;

#[tokio::test]
async fn truncates_children_beyond_their_limit() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let article = created_article(database, &author, "Shipping").await;
    let german = translated(database, &article, "de").await;
    let english = translated(database, &article, "en").await;

    translated(database, &article, "pl").await;

    let Lookup::Found(ArticleWithTranslations { translations, .. }) = Article::query()
        .id
        .eq(article.id)
        .load::<ArticleWithTranslations, _>(database)
        .await
        .expect("the article is loaded")
    else {
        panic!("the article is found");
    };

    assert_eq!(translations.completeness, Completeness::Truncated);
    assert_eq!(translations.records, vec![german, english]);
}

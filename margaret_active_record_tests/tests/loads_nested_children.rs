use margaret::framework::active_record::children::Children;
use margaret::framework::active_record::completeness::Completeness;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article_with_translations::ArticleWithTranslations;
use margaret_active_record_tests::models::author::Author;
use margaret_active_record_tests::models::author_with_articles::AuthorWithArticles;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;
use crate::translated::translated;

#[tokio::test]
async fn loads_nested_children() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let article = created_article(database, &author, "Shipping").await;
    let german = translated(database, &article, "de").await;
    let polish = translated(database, &article, "pl").await;

    assert_eq!(
        Author::query()
            .id
            .eq(author.id)
            .load::<AuthorWithArticles, _>(database)
            .await
            .expect("the author is loaded"),
        Lookup::Found(AuthorWithArticles {
            author,
            articles: Children {
                completeness: Completeness::Complete,
                records: vec![ArticleWithTranslations {
                    article,
                    translations: Children {
                        completeness: Completeness::Complete,
                        records: vec![german, polish],
                    },
                }],
            },
        })
    );
}

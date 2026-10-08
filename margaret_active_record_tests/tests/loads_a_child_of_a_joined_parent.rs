use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_with_author_profile::ArticleWithAuthorProfile;
use margaret_active_record_tests::models::author_with_profile::AuthorWithProfile;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::profiled::profiled;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn loads_a_child_of_a_joined_parent() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let profile = profiled(database, &author).await;
    let article = created_article(database, &author, "Shipping").await;

    assert_eq!(
        Article::query()
            .id
            .eq(article.id)
            .load::<ArticleWithAuthorProfile, _>(database)
            .await
            .expect("the article is loaded"),
        Lookup::Found(ArticleWithAuthorProfile {
            article,
            author: AuthorWithProfile {
                author,
                profile: Some(profile),
            },
        })
    );
}

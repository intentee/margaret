use std::sync::Arc;

use uuid::Uuid;

use margaret_example::forms::post_article_form::PostArticleForm;
use margaret_example::models::article::Article;
use margaret_example::routes::public::post_article::PostArticle;
use margaret_example::system_clock::SystemClock;
use margaret_example_tests::seeded_blog::seeded_blog;

async fn status_of(author_id: Uuid) -> u16 {
    let blog = seeded_blog().await;
    let responder = PostArticle::create(Arc::new(SystemClock), Arc::clone(&blog.database))
        .expect("the responder is constructed");

    responder
        .respond(PostArticleForm {
            title: "Title".to_string(),
            body: "Body".to_string(),
            author_id,
        })
        .await
        .expect("the responder succeeds")
        .status()
}

#[tokio::test]
async fn responds_with_500_when_the_author_is_unknown() {
    assert_eq!(status_of(Uuid::from_u128(999)).await, 500);
}

#[tokio::test]
async fn creates_a_draft_for_a_known_author() {
    let blog = seeded_blog().await;
    let responder = PostArticle::create(Arc::new(SystemClock), Arc::clone(&blog.database))
        .expect("the responder is constructed");

    assert_eq!(
        responder
            .respond(PostArticleForm {
                title: "Title".to_string(),
                body: "Body".to_string(),
                author_id: Uuid::from_u128(3),
            })
            .await
            .expect("the responder succeeds")
            .status(),
        201
    );
    assert!(
        Article::listed(&blog.database, Some("Milo".to_string()))
            .await
            .expect("the articles are listed")
            .iter()
            .any(|article| article.title == "Title")
    );
}

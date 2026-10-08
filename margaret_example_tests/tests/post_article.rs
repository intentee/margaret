use std::sync::Arc;

use uuid::Uuid;

use margaret_example::forms::post_article_form::PostArticleForm;
use margaret_example::routes::public::post_article::PostArticle;
use margaret_example::stores::article_store::ArticleStore;
use margaret_example::system_clock::SystemClock;
use margaret_example_tests::seeded_blog::seeded_blog;

#[tokio::test]
async fn responds_with_500_when_the_author_is_unknown() {
    let blog = seeded_blog().await;
    let store = ArticleStore::create(Arc::new(SystemClock), Arc::clone(&blog.database))
        .expect("the article store is constructed");
    let responder = PostArticle::create(Arc::new(store)).expect("the responder is constructed");
    let form = PostArticleForm {
        title: "Title".to_string(),
        body: "Body".to_string(),
        author_id: Uuid::from_u128(999),
    };

    assert_eq!(
        responder
            .respond(form)
            .await
            .expect("the responder succeeds")
            .status(),
        500
    );
}

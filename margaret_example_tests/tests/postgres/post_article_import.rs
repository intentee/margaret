use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::validation::validation_result::ValidationResult;
use margaret_example::forms::post_article_form::PostArticleForm;
use margaret_example::repositories::article_repository::ArticleRepository;
use margaret_example::routes::public::post_article_import::PostArticleImport;
use margaret_example::singletons::system_clock::SystemClock;
use margaret_example_tests::seeded_blog::seeded_blog;

#[tokio::test]
async fn responds_with_500_when_the_author_is_unknown() {
    let blog = seeded_blog().await;
    let responder = PostArticleImport::create(
        Arc::new(
            ArticleRepository::create(Arc::clone(&blog.database))
                .expect("the article repository is constructed"),
        ),
        Arc::new(SystemClock),
    )
    .expect("the responder is constructed");
    let form = ValidationResult::Valid(PostArticleForm {
        title: "Title".to_string(),
        body: "Body".to_string(),
        author_id: Uuid::from_u128(999),
    });

    assert_eq!(
        responder
            .respond(form)
            .await
            .expect("the responder succeeds")
            .status(),
        500
    );
}

use std::sync::Arc;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_example::featured_article_id::FEATURED_ARTICLE_ID;
use margaret_example::forms::patch_article_form::PatchArticleForm;
use margaret_example::models::article::Article;
use margaret_example::routes::public::patch_article::PatchArticle;
use margaret_example_tests::seeded_blog::seeded_blog;

#[tokio::test]
async fn saves_the_changes_of_an_article() {
    let blog = seeded_blog().await;
    let Lookup::Found(article) = Article::query()
        .id
        .eq(FEATURED_ARTICLE_ID)
        .find(blog.database.as_ref())
        .await
        .expect("the article is read")
    else {
        panic!("the featured article is seeded");
    };

    assert_eq!(
        PatchArticle::create(Arc::clone(&blog.database))
            .expect("the responder is constructed")
            .respond(
                article,
                PatchArticleForm {
                    title: Some("Shipping Margaret on Postgres".to_string()),
                    body: None,
                },
            )
            .await
            .expect("the responder succeeds")
            .status(),
        200
    );
    assert!(matches!(
        Article::query()
            .id
            .eq(FEATURED_ARTICLE_ID)
            .find(blog.database.as_ref())
            .await
            .expect("the article is read"),
        Lookup::Found(Article { title, body, .. })
            if title == "Shipping Margaret on Postgres" && body == "A public note from Milo."
    ));
}

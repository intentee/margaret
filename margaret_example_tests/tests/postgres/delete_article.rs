use std::sync::Arc;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_example::featured_article_id::FEATURED_ARTICLE_ID;
use margaret_example::models::article::Article;
use margaret_example::routes::public::delete_article::DeleteArticle;
use margaret_example_tests::seeded_blog::seeded_blog;

#[tokio::test]
async fn removes_an_article() {
    let blog = seeded_blog().await;
    let featured = || Article::query().id.eq(FEATURED_ARTICLE_ID);
    let Lookup::Found(article) = featured()
        .find(blog.database.as_ref())
        .await
        .expect("the article is read")
    else {
        panic!("the featured article is seeded");
    };

    assert_eq!(
        DeleteArticle::create(Arc::clone(&blog.database))
            .expect("the responder is constructed")
            .respond(article)
            .await
            .expect("the responder succeeds")
            .status(),
        200
    );
    assert!(matches!(
        featured()
            .find(blog.database.as_ref())
            .await
            .expect("the article is read"),
        Lookup::Missing
    ));
}

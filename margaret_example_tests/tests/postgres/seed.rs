use std::sync::Arc;

use margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder;
use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;
use margaret_example::commands::seed::Seed;
use margaret_example::featured_article_id::FEATURED_ARTICLE_ID;
use margaret_example::models::article::Article;
use margaret_example::models::article_with_author::ArticleWithAuthor;
use margaret_example::system_clock::SystemClock;
use margaret_example_tests::seeded_blog::seeded_blog;

#[tokio::test]
async fn binds_the_seeded_featured_article_with_its_author() {
    let blog = seeded_blog().await;
    let binder = PrimaryKeyBinder::<ArticleWithAuthor>::new(&blog.database);

    assert!(matches!(
        binder.bind(FEATURED_ARTICLE_ID.to_string()).await,
        Ok(RouteParameterBindingOutcome::Bound(ArticleWithAuthor { article, author }))
            if article.title == "Shipping Margaret" && author.name == "Milo"
    ));
    assert!(matches!(
        binder.bind("not-a-uuid".to_string()).await,
        Ok(RouteParameterBindingOutcome::NotFound)
    ));
}

#[tokio::test]
async fn seeds_the_blog_again_without_duplicating_it() {
    let blog = seeded_blog().await;

    assert!(matches!(
        Seed::create(Arc::new(SystemClock), Arc::clone(&blog.database))
            .expect("the seed command is constructed")
            .run()
            .await
            .expect("the blog is seeded again"),
        CommandOutcome::Succeeded
    ));
    assert_eq!(
        Article::listed(&blog.database, None)
            .await
            .expect("the articles are listed")
            .len(),
        3
    );
}

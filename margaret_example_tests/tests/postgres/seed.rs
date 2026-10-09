use std::num::NonZeroUsize;
use std::sync::Arc;

use margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder;
use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::database::max_connections::MaxConnections;
use margaret::framework::route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;
use margaret::framework::sessions::issued_sessions::IssuedSessions;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_example::commands::seed::Seed;
use margaret_example::featured_article_id::FEATURED_ARTICLE_ID;
use margaret_example::margaret::schema::SCHEMA;
use margaret_example::margaret::sessions::session_audience::SESSION_AUDIENCE;
use margaret_example::models::article::Article;
use margaret_example::models::article_with_author::ArticleWithAuthor;
use margaret_example::system_clock::SystemClock;
use margaret_example_tests::seeded_blog::seeded_blog;
use margaret_sessions_tests::session_store::session_store;

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
        Seed::create(
            Arc::new(SystemClock),
            Arc::clone(&blog.database),
            Arc::new(IssuedSessions::host_only(
                Arc::clone(&blog.database),
                session_store(),
                SESSION_AUDIENCE,
            )),
        )
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

#[tokio::test]
async fn seeds_the_blog_through_a_single_connection() {
    let started = StartedDatabase::with_schema(&SCHEMA).await;
    let database = started
        .separate_pool_of(MaxConnections {
            connections: NonZeroUsize::MIN,
        })
        .await;

    assert!(matches!(
        Seed::create(
            Arc::new(SystemClock),
            Arc::clone(&database),
            Arc::new(IssuedSessions::host_only(
                Arc::clone(&database),
                session_store(),
                SESSION_AUDIENCE,
            )),
        )
        .expect("the seed command is constructed")
        .run()
        .await
        .expect("the blog is seeded"),
        CommandOutcome::Succeeded
    ));
}

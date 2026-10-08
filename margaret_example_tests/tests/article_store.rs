use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::database::database::Database;
use margaret::framework::route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;
use margaret_example::stores::article_insertion::ArticleInsertion;
use margaret_example::stores::article_store::ArticleStore;
use margaret_example::stores::featured_article_id::FEATURED_ARTICLE_ID;
use margaret_example::system_clock::SystemClock;
use margaret_example_tests::seeded_blog::seeded_blog;

fn store(database: &Arc<Database>) -> ArticleStore {
    ArticleStore::create(Arc::new(SystemClock), Arc::clone(database))
        .expect("the article store is constructed")
}

#[tokio::test]
async fn seeds_the_featured_article_with_its_author() {
    let blog = seeded_blog().await;
    let article = store(&blog.database)
        .find_article_by_id(FEATURED_ARTICLE_ID)
        .await
        .expect("the article is read")
        .expect("the featured article is seeded");

    assert_eq!(article.title, "Shipping Margaret");
    assert_eq!(article.author.name, "Milo");
}

#[tokio::test]
async fn inserts_an_article_for_a_known_author() {
    let blog = seeded_blog().await;
    let store = store(&blog.database);
    let ArticleInsertion::Inserted(article) = store
        .insert("Title".to_string(), "Body".to_string(), Uuid::from_u128(3))
        .await
        .expect("the insertion is attempted")
    else {
        panic!("the article is inserted for a known author");
    };

    assert_eq!(article.author.name, "Milo");
    assert!(
        store
            .find_article_by_id(article.id)
            .await
            .expect("the article is read")
            .is_some()
    );
}

#[tokio::test]
async fn refuses_an_article_for_an_unknown_author() {
    let blog = seeded_blog().await;

    assert!(matches!(
        store(&blog.database)
            .insert(
                "Title".to_string(),
                "Body".to_string(),
                Uuid::from_u128(999)
            )
            .await
            .expect("the insertion is attempted"),
        ArticleInsertion::AuthorNotFound(refusal) if refusal.author_id == Uuid::from_u128(999)
    ));
}

#[tokio::test]
async fn saves_the_changes_of_an_article() {
    let blog = seeded_blog().await;
    let store = store(&blog.database);
    let mut article = store
        .find_article_by_id(FEATURED_ARTICLE_ID)
        .await
        .expect("the article is read")
        .expect("the featured article is seeded");

    article.title = "Shipping Margaret on Postgres".to_string();
    store.save(&article).await.expect("the article is saved");

    assert_eq!(
        store
            .find_article_by_id(FEATURED_ARTICLE_ID)
            .await
            .expect("the article is read")
            .map(|article| article.title),
        Some("Shipping Margaret on Postgres".to_string())
    );
}

#[tokio::test]
async fn removes_an_article() {
    let blog = seeded_blog().await;
    let store = store(&blog.database);

    store
        .remove(FEATURED_ARTICLE_ID)
        .await
        .expect("the article is removed");

    assert!(
        store
            .find_article_by_id(FEATURED_ARTICLE_ID)
            .await
            .expect("the article is read")
            .is_none()
    );
}

#[tokio::test]
async fn lists_the_seeded_articles_in_id_order() {
    let blog = seeded_blog().await;
    let all = store(&blog.database)
        .all()
        .await
        .expect("the articles are listed");

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].id, FEATURED_ARTICLE_ID);
}

#[tokio::test]
async fn binds_an_article_by_its_uuid() {
    let blog = seeded_blog().await;
    let store = store(&blog.database);

    assert!(matches!(
        store.bind(FEATURED_ARTICLE_ID.to_string()).await,
        Ok(RouteParameterBindingOutcome::Bound(_))
    ));
    assert!(matches!(
        store.bind("not-a-uuid".to_string()).await,
        Ok(RouteParameterBindingOutcome::NotFound)
    ));
}

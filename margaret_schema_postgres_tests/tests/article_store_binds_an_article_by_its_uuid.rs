use margaret::framework::http::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_example::stores::article_store::FEATURED_ARTICLE_ID;
use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::build_article_store::build_article_store;
use margaret_schema_postgres_tests::seed_demo_data::seed_demo_data;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn binds_an_article_by_its_uuid() {
    let database = start_database().await;
    apply_schema(database.pool()).await;
    seed_demo_data(database.pool()).await;
    let store = build_article_store(&database);

    assert!(
        store
            .bind(FEATURED_ARTICLE_ID.to_string())
            .await
            .expect("the featured article is queried")
            .is_some()
    );
    assert!(
        store
            .bind("not-a-uuid".to_string())
            .await
            .expect("a malformed id binds to nothing")
            .is_none()
    );
}

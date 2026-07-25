use margaret_example::stores::article_store::FEATURED_ARTICLE_ID;
use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::build_article_store::build_article_store;
use margaret_schema_postgres_tests::seed_demo_data::seed_demo_data;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn lists_articles_in_id_order() {
    let database = start_database().await;
    apply_schema(database.pool()).await;
    seed_demo_data(database.pool()).await;
    let store = build_article_store(&database);

    let all = store.all().await.expect("the seeded articles are listed");

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].id, FEATURED_ARTICLE_ID);
}

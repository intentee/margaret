use margaret_example::stores::article_store::FEATURED_ARTICLE_ID;
use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::build_article_store::build_article_store;
use margaret_schema_postgres_tests::seed_demo_data::seed_demo_data;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn reads_the_featured_article_with_its_author() {
    let database = start_database().await;
    apply_schema(database.pool()).await;
    seed_demo_data(database.pool()).await;
    let store = build_article_store(&database);

    let article = store
        .find_article_by_id(FEATURED_ARTICLE_ID)
        .await
        .expect("the featured article is queried")
        .expect("the featured article is present");

    assert_eq!(article.title, "Shipping Margaret");
    assert_eq!(article.author.name, "Milo");
}

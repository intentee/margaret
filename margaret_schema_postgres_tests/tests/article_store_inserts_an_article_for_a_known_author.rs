use uuid::Uuid;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::build_article_store::build_article_store;
use margaret_schema_postgres_tests::seed_demo_data::seed_demo_data;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn inserts_an_article_for_a_known_author() {
    let database = start_database().await;
    apply_schema(database.pool()).await;
    seed_demo_data(database.pool()).await;
    let store = build_article_store(&database);

    let article = store
        .insert("Title".to_string(), "Body".to_string(), Uuid::from_u128(3))
        .await
        .expect("the article is inserted for a known author");

    assert_eq!(article.author.name, "Milo");
    assert!(
        store
            .find_article_by_id(article.id)
            .await
            .expect("the inserted article is queried")
            .is_some()
    );
}

use uuid::Uuid;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::build_article_store::build_article_store;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn rejects_an_article_for_an_unknown_author() {
    let database = start_database().await;
    apply_schema(database.pool()).await;
    let store = build_article_store(&database);

    let result = store
        .insert(
            "Title".to_string(),
            "Body".to_string(),
            Uuid::from_u128(999),
        )
        .await;

    assert!(result.is_err());
}

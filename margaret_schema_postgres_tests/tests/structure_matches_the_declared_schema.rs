use margaret_example::margaret::schema::schema;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::assert_table_matches::assert_table_matches;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn the_applied_schema_matches_the_declared_structure() {
    let database = start_database().await;
    let pool = database.pool();
    let declared = schema();

    apply_schema(pool, &declared).await;

    for table in &declared.tables {
        assert_table_matches(pool, table).await;
    }
}

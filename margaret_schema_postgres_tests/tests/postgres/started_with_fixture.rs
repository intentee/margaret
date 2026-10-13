use margaret_database_tests::started_database::StartedDatabase;
use margaret_model::schema::Schema;
use margaret_schema_postgres_fixture::margaret::tables::TABLES;

pub async fn started_with_fixture() -> StartedDatabase {
    StartedDatabase::with_schema(&Schema {
        table_sets: &[TABLES],
    })
    .await
}

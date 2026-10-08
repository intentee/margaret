use margaret_client_assertions::margaret::tables::TABLES;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_model::schema::Schema;

pub async fn started_with_client_assertions() -> StartedDatabase {
    StartedDatabase::with_schema(&Schema {
        table_sets: &[TABLES],
    })
    .await
}

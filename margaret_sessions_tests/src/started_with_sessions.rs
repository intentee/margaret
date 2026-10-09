use margaret_database_tests::started_database::StartedDatabase;
use margaret_model::schema::Schema;
use margaret_sessions::margaret::tables::TABLES;

pub async fn started_with_sessions() -> StartedDatabase {
    StartedDatabase::with_schema(&Schema {
        table_sets: &[TABLES],
    })
    .await
}

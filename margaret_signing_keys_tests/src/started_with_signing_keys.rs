use margaret_database_tests::started_database::StartedDatabase;
use margaret_model::schema::Schema;
use margaret_signing_keys::margaret::tables::TABLES;

pub async fn started_with_signing_keys() -> StartedDatabase {
    StartedDatabase::with_schema(&Schema {
        table_sets: &[TABLES],
    })
    .await
}

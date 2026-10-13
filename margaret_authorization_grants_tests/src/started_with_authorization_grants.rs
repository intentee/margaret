use margaret_authorization_grants::margaret::tables::TABLES;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_model::schema::Schema;

pub async fn started_with_authorization_grants() -> StartedDatabase {
    StartedDatabase::with_schema(&Schema {
        table_sets: &[TABLES],
    })
    .await
}

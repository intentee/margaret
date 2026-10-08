use margaret_database_tests::started_database::StartedDatabase;
use margaret_model::schema::Schema;

pub async fn started_with_provider_state() -> StartedDatabase {
    StartedDatabase::with_schema(&Schema {
        table_sets: &[
            margaret_authorization_grants::margaret::tables::TABLES,
            margaret_client_assertions::margaret::tables::TABLES,
        ],
    })
    .await
}

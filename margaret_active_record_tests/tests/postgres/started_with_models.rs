use margaret::framework::model::schema::Schema;
use margaret_active_record_tests::margaret::tables::TABLES;
use margaret_database_tests::started_database::StartedDatabase;

pub async fn started_with_models() -> StartedDatabase {
    StartedDatabase::with_schema(&Schema {
        table_sets: &[TABLES],
    })
    .await
}

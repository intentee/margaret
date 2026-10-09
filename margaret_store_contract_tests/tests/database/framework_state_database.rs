use margaret_cluster_fixture::margaret::schema::schema;
use margaret_database_tests::apply_schema::apply_schema;
use margaret_database_tests::started_database::StartedDatabase;

pub async fn framework_state_database() -> StartedDatabase {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    started
}

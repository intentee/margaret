use margaret_database_tests::started_database::StartedDatabase;

pub async fn create_probes(started: &StartedDatabase) {
    started
        .execute("CREATE TABLE probes (id BIGINT PRIMARY KEY, amount BIGINT NOT NULL)")
        .await;
}

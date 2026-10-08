use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;

use crate::create_probes::create_probes;
use crate::select_probe_amounts::select_probe_amounts;

#[tokio::test]
async fn reads_an_optional_row() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;

    assert!(
        started
            .database
            .optional_row(&select_probe_amounts())
            .await
            .expect("the absent probe is read")
            .is_none()
    );
}

use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;

use crate::postgres::create_probes::create_probes;
use crate::postgres::select_probe_amounts::select_probe_amounts;

#[tokio::test]
async fn reads_no_optional_row_through_the_pool_when_none_matches_instead_of_failing() {
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

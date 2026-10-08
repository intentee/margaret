use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;

use crate::create_probes::create_probes;
use crate::insert_probe::insert_probe;
use crate::select_probe_amounts::select_probe_amounts;

#[tokio::test]
async fn reads_exactly_one_row() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;
    started
        .database
        .affected(&insert_probe(1, 10, ConflictAction::Raise))
        .await
        .expect("the probe is inserted");

    assert_eq!(
        started
            .database
            .row(&select_probe_amounts())
            .await
            .expect("the single probe is read")
            .get::<usize, i64>(0),
        10
    );
}

use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;

use crate::create_probes::create_probes;
use crate::insert_probe::insert_probe;
use crate::probe_amounts::probe_amounts;

#[tokio::test]
async fn executes_statements_through_the_pool() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;

    assert_eq!(
        started
            .database
            .affected(&insert_probe(1, 10, ConflictAction::Raise))
            .await
            .expect("the probe is inserted"),
        1
    );
    assert_eq!(probe_amounts(started.database.as_ref()).await, vec![10]);
}

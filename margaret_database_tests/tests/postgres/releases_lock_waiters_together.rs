use futures_util::future::join_all;

use margaret_database::executor::Executor;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;

use crate::postgres::create_probes::create_probes;
use crate::postgres::insert_probe::insert_probe;
use crate::postgres::probe_amounts::probe_amounts;
use crate::postgres::update_probe_amount::update_probe_amount;

const WAITERS: i64 = 3;

#[tokio::test]
async fn releases_lock_waiters_together() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;
    started
        .database
        .affected(&insert_probe(1, 0, ConflictAction::Raise))
        .await
        .expect("the probe is inserted");

    let mut connection = started
        .database
        .connection()
        .await
        .expect("a connection is checked out");
    let holder = connection
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the lock holding transaction begins");

    holder
        .affected(&update_probe_amount(1, 1))
        .await
        .expect("the holder locks the probe");

    let database = started.database.as_ref();
    let waiters = join_all((0..WAITERS).map(|amount| async move {
        database
            .affected(&update_probe_amount(1, 100 + amount))
            .await
    }));
    let released = async {
        started.administration.await_lock_waiters(WAITERS).await;
        holder.commit().await.expect("the holder releases the lock");
    };
    let (updated, ()) = tokio::join!(waiters, released);

    assert!(
        updated
            .into_iter()
            .all(|outcome| outcome.expect("every waiter updates the probe") == 1)
    );
    assert_eq!(probe_amounts(started.database.as_ref()).await.len(), 1);
}

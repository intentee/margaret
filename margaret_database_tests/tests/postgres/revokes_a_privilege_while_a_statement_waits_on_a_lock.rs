use tokio_postgres::error::SqlState;

use margaret_database::database_error::DatabaseError;
use margaret_database::executor::Executor;
use margaret_database::isolation::Isolation;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_sql::conflict_action::ConflictAction;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::postgres::create_probes::create_probes;
use crate::postgres::insert_probe::insert_probe;
use crate::postgres::update_probe_amount::update_probe_amount;

#[tokio::test]
async fn revokes_a_privilege_while_a_statement_waits_on_a_lock() {
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

    let blocked_update = update_probe_amount(1, 2);
    let pending_update = started.database.affected(&blocked_update);
    let revoked = async {
        started.administration.await_lock_waiters(1).await;
        started
            .administration
            .revoke(
                TablePrivilege::Update,
                TableNamespace::Application,
                "probes",
            )
            .await;
        holder.commit().await.expect("the holder releases the lock");
    };
    let (completed_update, ()) = tokio::join!(pending_update, revoked);

    assert_eq!(
        completed_update.expect("the waiting statement completes"),
        1
    );

    let Err(
        DatabaseError::StatementPreparation(source) | DatabaseError::StatementExecution(source),
    ) = started.database.affected(&update_probe_amount(1, 3)).await
    else {
        panic!("the revoked privilege refuses a later update");
    };

    assert_eq!(source.code(), Some(&SqlState::INSUFFICIENT_PRIVILEGE));
}

use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_sql::conflict_action::ConflictAction;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::create_probes::create_probes;
use crate::insert_probe::insert_probe;
use crate::probe_amounts::probe_amounts;

#[tokio::test]
async fn reports_a_conflict_with_a_row_hidden_by_row_level_security() {
    let started = StartedDatabase::start().await;

    create_probes(&started).await;
    started
        .database
        .affected(&insert_probe(1, 10, ConflictAction::Raise))
        .await
        .expect("the probe is inserted");
    started
        .administration
        .hide_rows(TableNamespace::Application, "probes", "amount <> 10")
        .await;

    assert!(probe_amounts(started.database.as_ref()).await.is_empty());
    assert_eq!(
        started
            .database
            .affected(&insert_probe(
                1,
                20,
                ConflictAction::Ignore { target: vec!["id"] }
            ))
            .await
            .expect("the conflicting insertion is ignored"),
        0
    );
}

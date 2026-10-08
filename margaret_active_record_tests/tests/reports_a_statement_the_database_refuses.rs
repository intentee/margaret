use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_active_record_tests::models::counter::Counter;
use margaret_database_tests::table_privilege::TablePrivilege;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_statement_the_database_refuses() {
    let started = started_with_models().await;

    started
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Application,
            "counters",
        )
        .await;

    assert!(matches!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .find(started.database.as_ref())
            .await,
        Err(ActiveRecordError::Database {
            statement: StatementKind::Select,
            table: "counters",
            ..
        })
    ));
}

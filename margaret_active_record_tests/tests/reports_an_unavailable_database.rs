use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::database::database_error::DatabaseError;
use margaret_active_record_tests::models::counter::Counter;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_an_unavailable_database() {
    let started = started_with_models().await;
    let held_connection = started
        .database
        .connection()
        .await
        .expect("the idle connection of the pool is checked out");

    started.administration.make_unreachable().await;

    assert!(matches!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .delete(started.database.as_ref())
            .await,
        Err(ActiveRecordError::Database {
            source: DatabaseError::Unavailable(_),
            statement: StatementKind::Delete,
            table: "counters",
        })
    ));

    drop(held_connection);
}

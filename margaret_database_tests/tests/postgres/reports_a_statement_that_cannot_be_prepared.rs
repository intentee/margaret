use tokio_postgres::error::SqlState;

use margaret_database::database_error::DatabaseError;
use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;

use crate::postgres::select_probe_amounts::select_probe_amounts;

#[tokio::test]
async fn reports_a_statement_that_cannot_be_prepared() {
    let started = StartedDatabase::start().await;
    let Err(DatabaseError::StatementPreparation(source)) =
        started.database.rows(&select_probe_amounts()).await
    else {
        panic!("a statement over a missing table is not prepared");
    };

    assert_eq!(source.code(), Some(&SqlState::UNDEFINED_TABLE));
    assert!(matches!(
        started.database.affected(&select_probe_amounts()).await,
        Err(DatabaseError::StatementPreparation(_))
    ));
    assert!(matches!(
        started.database.optional_row(&select_probe_amounts()).await,
        Err(DatabaseError::StatementPreparation(_))
    ));
    assert!(matches!(
        started.database.row(&select_probe_amounts()).await,
        Err(DatabaseError::StatementPreparation(_))
    ));
}

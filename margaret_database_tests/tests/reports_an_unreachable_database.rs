use margaret_database::database::Database;
use margaret_database::database_error::DatabaseError;
use margaret_database::executor::Executor;
use margaret_database_tests::started_database::StartedDatabase;

use crate::select_probe_amounts::select_probe_amounts;

#[tokio::test]
async fn reports_an_unreachable_database() {
    let started = StartedDatabase::start().await;
    let held_connection = started
        .database
        .connection()
        .await
        .expect("the idle connection of the pool is checked out");

    started.administration.make_unreachable().await;

    assert!(matches!(
        started.database.affected(&select_probe_amounts()).await,
        Err(DatabaseError::Unavailable(_))
    ));
    assert!(matches!(
        started.database.optional_row(&select_probe_amounts()).await,
        Err(DatabaseError::Unavailable(_))
    ));
    assert!(matches!(
        started.database.row(&select_probe_amounts()).await,
        Err(DatabaseError::Unavailable(_))
    ));
    assert!(matches!(
        started.database.rows(&select_probe_amounts()).await,
        Err(DatabaseError::Unavailable(_))
    ));
    assert!(matches!(
        started.database.connection().await,
        Err(DatabaseError::Unavailable(_))
    ));
    assert!(matches!(
        Database::connect(
            started
                .database_url
                .as_str()
                .parse()
                .expect("the test database url is a postgres url")
        )
        .await,
        Err(DatabaseError::Unavailable(_))
    ));

    drop(held_connection);
}

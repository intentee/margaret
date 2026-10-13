use chrono::Utc;
use uuid::Uuid;

use margaret_database_tests::started_database::StartedDatabase;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::sessions_error::SessionsError;
use margaret_sessions_tests::session_store::session_store;

#[tokio::test]
async fn reports_a_session_started_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE)
            .start(Uuid::new_v4(), Utc::now())
            .await,
        Err(SessionsError::SweepSessions(_))
    ));
}

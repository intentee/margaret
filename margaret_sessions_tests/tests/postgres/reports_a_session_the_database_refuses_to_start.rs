use chrono::Utc;
use uuid::Uuid;

use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::sessions_error::SessionsError;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_session_the_database_refuses_to_start() {
    let started = started_with_sessions().await;

    started
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "sessions",
        )
        .await;

    assert!(matches!(
        IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE)
            .start(Uuid::new_v4(), Utc::now())
            .await,
        Err(SessionsError::OpenSession(_))
    ));
}

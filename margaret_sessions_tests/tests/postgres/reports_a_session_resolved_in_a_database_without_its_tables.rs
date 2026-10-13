use cookie::Cookie;
use http::Method;

use margaret_database_tests::started_database::StartedDatabase;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::sessions_error::SessionsError;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;

#[tokio::test]
async fn reports_a_session_resolved_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE)
            .resolve(&presenting_cookies(
                Method::GET,
                &[Cookie::new("__Host-margaret-session", "contract-secret")],
                PeerIdentity::Anonymous,
            ))
            .await,
        Err(SessionsError::FindSession(_))
    ));
}

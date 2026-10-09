use http::Method;

use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn finds_no_session_without_session_cookies() {
    let started = started_with_sessions().await;
    let ResolvedSession {
        cookie_changes,
        session,
    } = IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE)
        .resolve(&presenting_cookies(
            Method::GET,
            &[],
            PeerIdentity::Anonymous,
        ))
        .await
        .expect("the request resolves");

    assert!(cookie_changes.cookies.is_empty());
    assert_eq!(session, None);
}

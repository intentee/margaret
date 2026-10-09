use chrono::Utc;
use http::Method;
use uuid::Uuid;

use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn resolves_a_started_session_from_its_access_token() {
    let started = started_with_sessions().await;
    let sessions =
        IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE);
    let begun = sessions
        .start(Uuid::new_v4(), Utc::now())
        .await
        .expect("the session starts");
    let ResolvedSession {
        cookie_changes,
        session,
    } = sessions
        .resolve(&presenting_cookies(
            Method::GET,
            &begun.cookie_changes.cookies,
            PeerIdentity::Anonymous,
        ))
        .await
        .expect("the session resolves");

    assert!(cookie_changes.cookies.is_empty());
    assert_eq!(session, Some(begun.session));
}

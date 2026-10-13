use cookie::Cookie;
use http::Method;

use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn clears_a_stale_access_token_presented_without_its_secret() {
    let started = started_with_sessions().await;
    let ResolvedSession {
        cookie_changes,
        session,
    } = IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE)
        .resolve(&presenting_cookies(
            Method::GET,
            &[Cookie::new("__Host-margaret-session-access", "stale")],
            PeerIdentity::Anonymous,
        ))
        .await
        .expect("the request resolves");

    assert_eq!(session, None);
    assert_eq!(
        cookie_changes
            .cookies
            .iter()
            .map(Cookie::name)
            .collect::<Vec<&str>>(),
        ["__Host-margaret-session-access", "__Host-margaret-session"]
    );
}

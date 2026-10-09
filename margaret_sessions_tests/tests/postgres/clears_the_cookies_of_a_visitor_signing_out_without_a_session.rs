use cookie::Cookie;
use http::Method;

use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn clears_the_cookies_of_a_visitor_signing_out_without_a_session() {
    let started = started_with_sessions().await;

    assert_eq!(
        IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE)
            .sign_out(&presenting_cookies(
                Method::POST,
                &[],
                PeerIdentity::Anonymous
            ))
            .await
            .expect("the visitor signs out")
            .cookies
            .iter()
            .map(Cookie::name)
            .collect::<Vec<&str>>(),
        ["__Host-margaret-session-access", "__Host-margaret-session"]
    );
}

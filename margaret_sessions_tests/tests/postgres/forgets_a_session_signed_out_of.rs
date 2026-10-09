use chrono::Utc;
use http::Method;
use uuid::Uuid;

use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn forgets_a_session_signed_out_of() {
    let started = started_with_sessions().await;
    let sessions =
        IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE);
    let begun = sessions
        .start(Uuid::new_v4(), Utc::now())
        .await
        .expect("the session starts");
    let secret = begun
        .cookie_changes
        .cookies
        .iter()
        .find(|cookie| cookie.name() == "__Host-margaret-session")
        .expect("the session secret is set")
        .clone();

    sessions
        .sign_out(&presenting_cookies(
            Method::POST,
            std::slice::from_ref(&secret),
            PeerIdentity::Anonymous,
        ))
        .await
        .expect("the session is signed out of");

    assert_eq!(
        sessions
            .resolve(&presenting_cookies(
                Method::GET,
                &[secret],
                PeerIdentity::Anonymous
            ))
            .await
            .expect("the request resolves")
            .session,
        None
    );
}

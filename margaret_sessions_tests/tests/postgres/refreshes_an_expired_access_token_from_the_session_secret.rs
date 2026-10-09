use chrono::DateTime;
use chrono::Utc;
use cookie::Cookie;
use http::Method;
use uuid::Uuid;

use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn refreshes_an_expired_access_token_from_the_session_secret() {
    let started = started_with_sessions().await;
    let store = session_store();
    let sessions =
        IssuedSessions::host_only(started.database.clone(), store.clone(), FIXTURE_AUDIENCE);
    let begun = sessions
        .start(Uuid::new_v4(), Utc::now())
        .await
        .expect("the session starts");
    let expired = store.issue_session_access_token(
        &SessionAccessTokenClaims {
            auth_time: begun.session.authenticated_at,
            sid: begun.session.id,
            sub: begun.session.subject,
        },
        FIXTURE_AUDIENCE,
        DateTime::from_timestamp(1_000, 0).expect("the moment is representable"),
    );
    let ResolvedSession {
        cookie_changes,
        session,
    } = sessions
        .resolve(&presenting_cookies(
            Method::GET,
            &[
                begun
                    .cookie_changes
                    .cookies
                    .iter()
                    .find(|cookie| cookie.name() == "__Host-margaret-session")
                    .expect("the session secret is set")
                    .clone(),
                Cookie::new("__Host-margaret-session-access", expired.signed_claims),
            ],
            PeerIdentity::Anonymous,
        ))
        .await
        .expect("the session resolves");

    assert_eq!(session, Some(begun.session));
    assert!(matches!(
        cookie_changes.cookies.as_slice(),
        [refreshed] if refreshed.name() == "__Host-margaret-session-access" && !refreshed.value().is_empty()
    ));
}

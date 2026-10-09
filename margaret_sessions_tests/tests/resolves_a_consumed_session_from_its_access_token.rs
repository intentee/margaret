use cookie::Cookie;
use http::Method;

use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_sessions::session::Session;
use margaret_sessions::session_resolution::SessionResolution;
use margaret_sessions_tests::consumed_sessions_of::consumed_sessions_of;
use margaret_sessions_tests::fixture_session_claims::fixture_session_claims;
use margaret_sessions_tests::fixture_session_token::fixture_session_token;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn resolves_a_consumed_session_from_its_access_token() {
    let store = session_store();
    let claims = fixture_session_claims();
    let access_token = fixture_session_token(&store, &claims);
    let expected = Session::from(claims);
    let resolution = consumed_sessions_of(
        TrustedIssuer::own(store, fixture_trust()),
        &TlsFixture::generate(),
        1,
    )
    .resolve(&presenting_cookies(
        Method::GET,
        &[Cookie::new(
            "__Secure-margaret-session-access",
            access_token,
        )],
        PeerIdentity::Anonymous,
    ))
    .await;

    assert!(matches!(
        resolution,
        SessionResolution::Resolved(ResolvedSession { cookie_changes, session: Some(session) })
            if cookie_changes.cookies.is_empty() && session == expected
    ));
}

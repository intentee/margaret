use cookie::Cookie;
use http::Method;

use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_sessions::session_resolution::SessionResolution;
use margaret_sessions_tests::consumed_sessions_of::consumed_sessions_of;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn clears_a_stale_consumed_access_token_presented_without_its_secret() {
    assert!(matches!(
        consumed_sessions_of(
            TrustedIssuer::own(session_store(), fixture_trust()),
            &TlsFixture::generate(),
            1,
        )
        .resolve(&presenting_cookies(
            Method::GET,
            &[Cookie::new("__Secure-margaret-session-access", "stale")],
            PeerIdentity::Anonymous,
        ))
        .await,
        SessionResolution::Resolved(ResolvedSession { cookie_changes, session: None })
            if cookie_changes
                .cookies
                .iter()
                .map(Cookie::name)
                .eq(["__Secure-margaret-session-access", "__Secure-margaret-session"])
    ));
}

use cookie::Cookie;
use http::Method;

use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_sessions::session_resolution::SessionResolution;
use margaret_sessions_tests::consumed_sessions_of::consumed_sessions_of;
use margaret_sessions_tests::fixture_refresh_issuer::FixtureRefreshIssuer;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn clears_the_cookies_of_a_secret_its_issuer_refuses() {
    let issuer = FixtureRefreshIssuer::answering(401, Vec::new()).await;
    let resolution = consumed_sessions_of(
        TrustedIssuer::own(session_store(), fixture_trust()),
        &issuer.tls,
        issuer.server.port(),
    )
    .resolve(&presenting_cookies(
        Method::GET,
        &[Cookie::new("__Secure-margaret-session", "forgotten")],
        PeerIdentity::Anonymous,
    ))
    .await;

    issuer.server.stop().await;

    assert!(matches!(
        resolution,
        SessionResolution::Resolved(ResolvedSession { cookie_changes, session: None })
            if cookie_changes
                .cookies
                .iter()
                .map(Cookie::name)
                .eq(["__Secure-margaret-session-access", "__Secure-margaret-session"])
    ));
}

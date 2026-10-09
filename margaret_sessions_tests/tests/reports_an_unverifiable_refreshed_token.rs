use cookie::Cookie;
use http::Method;

use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::session_resolution::SessionResolution;
use margaret_sessions::session_unavailability::SessionUnavailability;
use margaret_sessions_tests::consumed_sessions_of::consumed_sessions_of;
use margaret_sessions_tests::fixture_refresh_issuer::FixtureRefreshIssuer;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_an_unverifiable_refreshed_token() {
    let issuer =
        FixtureRefreshIssuer::answering(200, br#"{"access_token":"unsigned"}"#.to_vec()).await;
    let resolution = consumed_sessions_of(
        TrustedIssuer::own(session_store(), fixture_trust()),
        &issuer.tls,
        issuer.server.port(),
    )
    .resolve(&presenting_cookies(
        Method::GET,
        &[Cookie::new("__Secure-margaret-session", "secret")],
        PeerIdentity::Anonymous,
    ))
    .await;

    issuer.server.stop().await;

    assert!(matches!(
        &resolution,
        SessionResolution::Unavailable(unavailability @ SessionUnavailability::UnverifiableRefreshedToken(rejection))
            if unavailability.to_string()
                == format!("the session issuer refreshed the session with an access token that does not verify: {rejection}")
    ));
}

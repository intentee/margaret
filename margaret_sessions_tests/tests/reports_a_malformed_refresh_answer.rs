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
async fn reports_a_malformed_refresh_answer() {
    let issuer = FixtureRefreshIssuer::answering(200, b"refreshed".to_vec()).await;
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
        SessionResolution::Unavailable(unavailability @ SessionUnavailability::MalformedRefreshAnswer { source })
            if unavailability.to_string()
                == format!("the session issuer answered the refresh with a malformed body: {source}")
    ));
}

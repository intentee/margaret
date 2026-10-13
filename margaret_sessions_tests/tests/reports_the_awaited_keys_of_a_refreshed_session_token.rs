use std::sync::Arc;

use cookie::Cookie;
use http::Method;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::session_resolution::SessionResolution;
use margaret_sessions::session_unavailability::SessionUnavailability;
use margaret_sessions_tests::consumed_sessions_of::consumed_sessions_of;
use margaret_sessions_tests::fixture_refresh_issuer::FixtureRefreshIssuer;
use margaret_sessions_tests::fixture_session_claims::fixture_session_claims;
use margaret_sessions_tests::fixture_session_token::fixture_session_token;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_the_awaited_keys_of_a_refreshed_session_token() {
    let access_token = fixture_session_token(&session_store(), &fixture_session_claims());
    let issuer = FixtureRefreshIssuer::answering(
        200,
        serde_json::to_vec(&serde_json::json!({ "access_token": access_token }))
            .expect("the refresh answer serializes"),
    )
    .await;
    let resolution = consumed_sessions_of(
        TrustedIssuer::polled(Arc::new(IssuerKeySet::awaiting()), fixture_trust()),
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
        resolution,
        SessionResolution::Unavailable(SessionUnavailability::KeysAwaited)
    ));
}

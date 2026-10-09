use std::sync::Arc;

use cookie::Cookie;
use http::Method;

use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::session_resolution::SessionResolution;
use margaret_sessions::session_unavailability::SessionUnavailability;
use margaret_sessions_tests::consumed_sessions_of::consumed_sessions_of;
use margaret_sessions_tests::fixture_session_claims::fixture_session_claims;
use margaret_sessions_tests::fixture_session_token::fixture_session_token;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_the_awaited_keys_of_a_session_issuer() {
    let access_token = fixture_session_token(&session_store(), &fixture_session_claims());
    let resolution = consumed_sessions_of(
        TrustedIssuer::polled(Arc::new(IssuerKeySet::awaiting()), fixture_trust()),
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
        &resolution,
        SessionResolution::Unavailable(unavailability @ SessionUnavailability::KeysAwaited)
            if unavailability.to_string() == "the signing keys of the session issuer are not available yet"
    ));
}

use cookie::Cookie;
use http::Method;

use margaret_http::response_continuation::ResponseContinuation;
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
async fn reports_an_issuer_answering_the_refresh_with_an_unexpected_status() {
    let issuer = FixtureRefreshIssuer::answering(500, Vec::new()).await;
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

    let SessionResolution::Unavailable(unavailability) = resolution else {
        panic!("the issuer is unavailable");
    };

    assert!(matches!(
        unavailability,
        SessionUnavailability::RefreshStatus { status } if status.as_u16() == 500
    ));
    assert_eq!(
        unavailability.to_string(),
        "the session issuer answered the refresh with status 500 Internal Server Error"
    );
    assert!(matches!(
        ResponseContinuation::from(unavailability),
        ResponseContinuation::Done(response) if response.status() == 503
    ));
}

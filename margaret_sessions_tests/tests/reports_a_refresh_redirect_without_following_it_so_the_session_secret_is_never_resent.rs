use std::sync::Arc;

use cookie::Cookie;
use http::Method;
use http::StatusCode;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_route_method::route_method::RouteMethod;
use margaret_sessions::session_resolution::SessionResolution;
use margaret_sessions::session_unavailability::SessionUnavailability;
use margaret_sessions_tests::consumed_sessions_of::consumed_sessions_of;
use margaret_sessions_tests::fixture_refresh_issuer::FixtureRefreshIssuer;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_a_refresh_redirect_without_following_it_so_the_session_secret_is_never_resent() {
    let plaintext = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        ["/sessions/refresh"],
        [RouteEntry::new(vec![MethodHandler::head(
            RouteMethod::Post,
            Arc::new(StaticHandler {
                body: b"{}".to_vec(),
                content_type: "application/json",
                status: 200,
            }),
        )])],
    )
    .await;
    let issuer = FixtureRefreshIssuer::redirecting(format!(
        "http://{}/sessions/refresh",
        plaintext.address()
    ))
    .await;
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
    plaintext.stop().await;

    assert!(matches!(
        resolution,
        SessionResolution::Unavailable(SessionUnavailability::RefreshStatus { status })
            if status == StatusCode::TEMPORARY_REDIRECT
    ));
}

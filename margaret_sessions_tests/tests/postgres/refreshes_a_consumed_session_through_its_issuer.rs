use std::sync::Arc;

use chrono::Utc;
use cookie::Cookie;
use http::Method;
use reqwest::Client;
use uuid::Uuid;

use margaret_http::body_limit::BodyLimit;
use margaret_http::limited_content_handler::limited_content_handler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_route_method::content_method::ContentMethod;
use margaret_sessions::consumed_sessions::ConsumedSessions;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_sessions::session_refresh_endpoint::SessionRefreshEndpoint;
use margaret_sessions::session_resolution::SessionResolution;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refreshes_a_consumed_session_through_its_issuer() {
    let started = started_with_sessions().await;
    let store = session_store();
    let issued = Arc::new(IssuedSessions::host_only(
        started.database.clone(),
        store.clone(),
        FIXTURE_AUDIENCE,
    ));
    let begun = issued
        .start(Uuid::new_v4(), Utc::now())
        .await
        .expect("the session starts");
    let secret = begun
        .cookie_changes
        .cookies
        .iter()
        .find(|cookie| cookie.name() == "__Host-margaret-session")
        .expect("the session secret is set")
        .value()
        .to_string();
    let mtls = MtlsFixture::new();
    let issuer_server = RunningFixtureServer::start(
        mtls.server_config.clone(),
        vec![RouteEntry::new(
            "/sessions/refresh",
            vec![MethodHandler::content(
                ContentMethod::Post,
                limited_content_handler(
                    Arc::new(SessionRefreshEndpoint::create(issued)),
                    BodyLimit::new(4_096),
                ),
            )],
        )],
    )
    .await;
    let consumed = ConsumedSessions::create(
        Arc::new(TrustedIssuer::own(store, fixture_trust())),
        "localhost".parse().expect("the domain is read"),
        format!(
            "https://localhost:{}/sessions/refresh",
            issuer_server.port()
        )
        .parse()
        .expect("the refresh url is read"),
        Client::builder()
            .use_preconfigured_tls(mtls.client_config.as_ref().clone())
            .build()
            .expect("the workload client builds"),
    );

    let resolution = consumed
        .resolve(&presenting_cookies(
            Method::GET,
            &[Cookie::new("__Secure-margaret-session", secret)],
            PeerIdentity::Anonymous,
        ))
        .await;

    issuer_server.stop().await;

    assert!(matches!(
        resolution,
        SessionResolution::Resolved(ResolvedSession {
            cookie_changes,
            session: Some(session),
        }) if session == begun.session
            && matches!(
                cookie_changes.cookies.as_slice(),
                [refreshed] if refreshed.name() == "__Secure-margaret-session-access"
                    && refreshed.domain() == Some("localhost")
            )
    ));
}

use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::hanging_handler::HangingHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_jwks_client::jwks_client::JwksClient;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;

#[tokio::test(flavor = "multi_thread")]
async fn jwks_client_returns_promptly_when_cancelled_during_a_hung_fetch() {
    let fixture = TlsFixture::generate();
    let hanging = Arc::new(HangingHandler::default());
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            WELL_KNOWN_JWKS_PATH,
            vec![MethodHandler::anonymous("GET", hanging.clone())],
        )],
    )
    .await;
    let jwks_url = fixture.url(server.port(), WELL_KNOWN_JWKS_PATH);
    let jwks_client = JwksClient::create(Arc::new(StaticEndpoint::new(jwks_url)));
    let client_builder = fixture_client_builder(&fixture.certificate_authority);
    let cancellation_token = CancellationToken::new();
    let poll_token = cancellation_token.clone();
    let poll_task = tokio::spawn(async move {
        jwks_client
            .run_with_client_builder(client_builder, poll_token)
            .await
    });

    hanging.request_received.cancelled().await;
    cancellation_token.cancel();

    poll_task
        .await
        .expect("the poll task joins")
        .expect("the poll service shuts down cleanly");

    hanging.release.cancel();
    server.stop().await;
}

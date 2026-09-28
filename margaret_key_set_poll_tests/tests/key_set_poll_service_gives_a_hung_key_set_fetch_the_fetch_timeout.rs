use std::sync::Arc;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::hanging_handler::HangingHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_key_set_poll::key_set_poll_fetch_timeout::KEY_SET_POLL_FETCH_TIMEOUT;
use margaret_key_set_poll::key_set_poll_service::KeySetPollService;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_key_set_poll_tests::first_tick_context::first_tick_context;
use margaret_key_set_poll_tests::fixed_key_set_locator::FixedKeySetLocator;

#[tokio::test(start_paused = true)]
async fn key_set_poll_service_gives_a_hung_key_set_fetch_the_fetch_timeout() {
    let fixture = TlsFixture::generate();
    let hanging = Arc::new(HangingHandler::default());
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/jwks",
            vec![MethodHandler::anonymous("GET", hanging.clone())],
        )],
    )
    .await;
    let mut service = KeySetPollService {
        issuer_document_client: IssuerDocumentClient::build(
            fixture_client_builder(&fixture.certificate_authority)
                .resolve(&fixture.server_name, server.address()),
        )
        .expect("the issuer document client builds"),
        locator: FixedKeySetLocator {
            key_set_url: fixture.url(server.port(), "/jwks"),
        },
        verification_key_set_holder: VerificationKeySetHolder::default(),
    };
    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), KEY_SET_POLL_FETCH_TIMEOUT);

    hanging.release.cancel();
    server.stop().await;
}

use std::sync::Arc;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::hanging_handler::HangingHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_issuer_request::issuer_request_timeout::ISSUER_REQUEST_TIMEOUT;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test(start_paused = true)]
async fn times_out_a_fetch_that_outlives_its_timeout() {
    let fixture = TlsFixture::generate();
    let hanging = Arc::new(HangingHandler::default());
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/document",
            vec![MethodHandler::anonymous(RouteMethod::Get, hanging.clone())],
        )],
    )
    .await;
    let client = IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
        .expect("the issuer request client builds");

    let started_at = Instant::now();
    let fetched = client
        .fetch_document(
            fixture.url(server.port(), "/document"),
            &CancellationToken::new(),
        )
        .await;

    assert!(matches!(
        fetched,
        IssuerDocument::TransportFailed(error) if error.is_timeout()
    ));
    assert_eq!(started_at.elapsed(), ISSUER_REQUEST_TIMEOUT);

    hanging.release.cancel();
    server.stop().await;
}

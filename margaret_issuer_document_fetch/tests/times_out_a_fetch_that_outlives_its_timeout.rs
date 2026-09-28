use std::sync::Arc;
use std::time::Duration;

use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::hanging_handler::HangingHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_issuer_document_fetch::issuer_document_fetch::IssuerDocumentFetch;
use margaret_issuer_document_fetch::issuer_document_request::IssuerDocumentRequest;

#[tokio::test]
async fn times_out_a_fetch_that_outlives_its_timeout() {
    let fixture = TlsFixture::generate();
    let hanging = Arc::new(HangingHandler::default());
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/document",
            vec![MethodHandler::anonymous("GET", hanging.clone())],
        )],
    )
    .await;
    let client =
        IssuerDocumentClient::build(fixture_client_builder(&fixture.certificate_authority))
            .expect("the issuer document client builds");

    let fetched = client
        .fetch(IssuerDocumentRequest {
            cancellation_token: &CancellationToken::new(),
            timeout: Duration::ZERO,
            url: fixture.url(server.port(), "/document"),
        })
        .await;

    assert!(matches!(
        fetched,
        IssuerDocumentFetch::TransportFailed(error) if error.is_timeout()
    ));

    hanging.release.cancel();
    server.stop().await;
}

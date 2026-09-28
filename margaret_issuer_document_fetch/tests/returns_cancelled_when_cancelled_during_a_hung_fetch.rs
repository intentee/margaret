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
async fn returns_cancelled_when_cancelled_during_a_hung_fetch() {
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
    let cancellation_token = CancellationToken::new();

    let (fetched, ()) = tokio::join!(
        client.fetch(IssuerDocumentRequest {
            cancellation_token: &cancellation_token,
            timeout: Duration::MAX,
            url: fixture.url(server.port(), "/document"),
        }),
        async {
            hanging.request_received.cancelled().await;
            cancellation_token.cancel();
        },
    );

    assert!(matches!(fetched, IssuerDocumentFetch::Cancelled));

    hanging.release.cancel();
    server.stop().await;
}

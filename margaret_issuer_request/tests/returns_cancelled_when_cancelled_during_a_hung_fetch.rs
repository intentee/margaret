use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::hanging_handler::HangingHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn returns_cancelled_when_cancelled_during_a_hung_fetch() {
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
    let cancellation_token = CancellationToken::new();

    let (fetched, ()) = tokio::join!(
        client.fetch_document(fixture.url(server.port(), "/document"), &cancellation_token),
        async {
            hanging.request_received.cancelled().await;
            cancellation_token.cancel();
        },
    );

    assert!(matches!(fetched, IssuerDocument::Cancelled));

    hanging.release.cancel();
    server.stop().await;
}

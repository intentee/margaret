use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn fetches_a_document_over_tls_without_client_authentication() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/document",
            vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(StaticHandler {
                    body: br#"{"keys":[]}"#.to_vec(),
                    content_type: "application/json",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let client = IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
        .expect("the issuer request client builds");

    let fetched = client
        .fetch_document(
            fixture.url(server.port(), "/document"),
            &CancellationToken::new(),
        )
        .await;

    let IssuerDocument::Fetched(document) = fetched else {
        panic!("the document is fetched, got {fetched:?}");
    };

    assert_eq!(document.as_ref(), br#"{"keys":[]}"#);

    server.stop().await;
}

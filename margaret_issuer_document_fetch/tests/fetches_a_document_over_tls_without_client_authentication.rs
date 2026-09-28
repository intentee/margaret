use std::sync::Arc;
use std::time::Duration;

use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_issuer_document_fetch::issuer_document_fetch::IssuerDocumentFetch;
use margaret_issuer_document_fetch::issuer_document_request::IssuerDocumentRequest;

#[tokio::test]
async fn fetches_a_document_over_tls_without_client_authentication() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/document",
            vec![MethodHandler::anonymous(
                "GET",
                Arc::new(StaticHandler {
                    body: br#"{"keys":[]}"#.to_vec(),
                    content_type: "application/json",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let client =
        IssuerDocumentClient::build(fixture_client_builder(&fixture.certificate_authority))
            .expect("the issuer document client builds");

    let fetched = client
        .fetch(IssuerDocumentRequest {
            cancellation_token: &CancellationToken::new(),
            timeout: Duration::MAX,
            url: fixture.url(server.port(), "/document"),
        })
        .await;

    let IssuerDocumentFetch::Fetched(document) = fetched else {
        panic!("the document is fetched, got {fetched:?}");
    };

    assert_eq!(document.as_ref(), br#"{"keys":[]}"#);

    server.stop().await;
}

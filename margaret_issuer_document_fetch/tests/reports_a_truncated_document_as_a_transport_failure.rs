use std::time::Duration;

use tokio_util::sync::CancellationToken;

use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_http_tests::truncated_response_server::TruncatedResponseServer;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_issuer_document_fetch::issuer_document_fetch::IssuerDocumentFetch;
use margaret_issuer_document_fetch::issuer_document_request::IssuerDocumentRequest;

#[tokio::test]
async fn reports_a_truncated_document_as_a_transport_failure() {
    let fixture = TlsFixture::generate();
    let server = TruncatedResponseServer::start(fixture.server_config.clone()).await;
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

    assert!(matches!(
        fetched,
        IssuerDocumentFetch::TransportFailed(error) if error.is_decode()
    ));

    server.finish().await;
}

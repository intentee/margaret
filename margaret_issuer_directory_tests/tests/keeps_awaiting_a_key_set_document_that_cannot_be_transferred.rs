use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_http_tests::truncated_response_server::TruncatedResponseServer;
use margaret_issuer_directory_tests::localhost_jwks_endpoint::localhost_jwks_endpoint;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_a_key_set_document_that_cannot_be_transferred() {
    let fixture = TlsFixture::generate();
    let server = TruncatedResponseServer::start(fixture.server_config.clone()).await;

    assert!(matches!(
        localhost_jwks_endpoint()
            .first_poll(
                IssuerRequestClient::build(
                    fixture_client_builder(&fixture.certificate_authority)
                        .resolve(&fixture.server_name, server.address())
                )
                .expect("the fixture request client builds")
            )
            .await,
        KeySetRefresh::Unchanged
    ));

    server.finish().await;
}

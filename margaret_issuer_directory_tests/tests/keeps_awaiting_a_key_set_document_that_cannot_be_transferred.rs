use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_http_tests::truncated_response_server::TruncatedResponseServer;
use margaret_issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer;
use margaret_issuer_directory_tests::polled_fixture::PolledFixture;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_a_key_set_document_that_cannot_be_transferred() {
    let fixture = TlsFixture::generate();
    let server = TruncatedResponseServer::start(fixture.server_config.clone()).await;
    let polled = PolledFixture::published(JwksEndpointIssuer {
        issuer: "https://localhost",
        jwks_uri: String::leak(fixture.url(server.port(), "/jwks").to_string()),
    });

    assert!(matches!(
        polled
            .first_poll(
                IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
                    .expect("the fixture request client builds")
            )
            .await,
        KeySetRefresh::Unchanged
    ));

    server.finish().await;
}

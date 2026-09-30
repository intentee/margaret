use reqwest::Method;
use reqwest::Request;

use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_http_tests::truncated_response_server::TruncatedResponseServer;
use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[tokio::test]
async fn reports_a_truncated_answer_as_a_transport_failure() {
    let fixture = TlsFixture::generate();
    let server = TruncatedResponseServer::start(fixture.server_config.clone()).await;
    let client = IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
        .expect("the issuer request client builds");
    let request = Request::new(Method::POST, fixture.url(server.port(), "/token"));

    assert!(matches!(
        client.exchange(request).await,
        Err(IssuerExchangeError::Transport(error)) if error.is_decode()
    ));

    server.finish().await;
}

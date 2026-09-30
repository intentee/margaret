use std::sync::Arc;

use reqwest::Method;
use reqwest::Request;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_issuer_request::issuer_response_max_bytes::ISSUER_RESPONSE_MAX_BYTES;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn refuses_an_answer_over_the_size_limit() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/token",
            vec![MethodHandler::anonymous(
                RouteMethod::Post,
                Arc::new(StaticHandler {
                    body: vec![b' '; ISSUER_RESPONSE_MAX_BYTES + 1],
                    content_type: "application/json",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let client = IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
        .expect("the issuer request client builds");
    let request = Request::new(Method::POST, fixture.url(server.port(), "/token"));

    let answer = client.exchange(request).await;

    server.stop().await;

    assert!(matches!(
        answer,
        Err(IssuerExchangeError::Oversized {
            max_bytes: ISSUER_RESPONSE_MAX_BYTES
        })
    ));
}

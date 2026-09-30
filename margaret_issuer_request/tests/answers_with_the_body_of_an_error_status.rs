use std::sync::Arc;

use http::HeaderValue;
use http::header::CONTENT_TYPE;
use reqwest::Method;
use reqwest::Request;

use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn answers_with_the_body_of_an_error_status() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/token",
            vec![MethodHandler::anonymous(
                RouteMethod::Post,
                Arc::new(FormEchoHandler {
                    limit: BodyLimit::new(1),
                    wrapping: EchoWrapping::Bare,
                }),
            )],
        )],
    )
    .await;
    let client = IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
        .expect("the issuer request client builds");
    let mut request = Request::new(Method::POST, fixture.url(server.port(), "/token"));

    request.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    *request.body_mut() = Some(b"grant_type=client_credentials".as_slice().into());

    let answer = client.exchange(request).await.expect("the issuer answers");

    server.stop().await;

    assert_eq!(answer.status(), 413);
    assert!(!answer.body().is_empty());
}

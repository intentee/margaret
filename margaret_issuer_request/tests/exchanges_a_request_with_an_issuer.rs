use std::sync::Arc;

use http::HeaderValue;
use http::header::AUTHORIZATION;
use http::header::CONTENT_TYPE;
use reqwest::Method;
use reqwest::Request;
use serde_json::Value;
use serde_json::json;

use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_route_method::content_method::ContentMethod;

#[tokio::test]
async fn exchanges_a_request_with_an_issuer() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/token",
            vec![MethodHandler::content(
                ContentMethod::Post,
                Arc::new(FormEchoHandler {
                    limit: BodyLimit::new(1024),
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
        AUTHORIZATION,
        HeaderValue::from_static("Basic Y2xpZW50OnNlY3JldA=="),
    );
    request.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    *request.body_mut() = Some(b"grant_type=client_credentials".as_slice().into());

    let answer = client.exchange(request).await.expect("the issuer answers");

    server.stop().await;

    let echoed: Value = serde_json::from_slice(answer.body()).expect("the echo is json");

    assert_eq!(answer.status(), 200);
    assert_eq!(
        answer
            .headers()
            .get(CONTENT_TYPE)
            .map(http::HeaderValue::as_bytes),
        Some(b"application/json".as_slice())
    );
    assert_eq!(
        echoed,
        json!({
            "authorization": "Basic Y2xpZW50OnNlY3JldA==",
            "fields": { "grant_type": "client_credentials" },
        })
    );
}

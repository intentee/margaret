use reqwest::header::AUTHORIZATION;
use reqwest::header::HeaderValue;
use reqwest::header::WWW_AUTHENTICATE;

use margaret_http_tests::echo_bearer_route::echo_bearer_route;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;

#[tokio::test]
async fn challenges_a_request_with_malformed_credentials() {
    let fixture = TlsFixture::generate();
    let server =
        RunningFixtureServer::start(fixture.server_config.clone(), vec![echo_bearer_route()]).await;
    let client = fixture_client_builder(&fixture.certificate_authority)
        .build()
        .expect("the fixture client builds");

    let response = client
        .get(fixture.url(server.port(), "/"))
        .header(AUTHORIZATION, "Bearer ab=c")
        .send()
        .await
        .expect("the fixture server answers");

    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        response.headers().get(WWW_AUTHENTICATE),
        Some(&HeaderValue::from_static(
            "Bearer error=\"invalid_request\""
        ))
    );

    server.stop().await;
}

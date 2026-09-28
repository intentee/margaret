use reqwest::header::AUTHORIZATION;

use margaret_http_tests::echo_bearer_route::echo_bearer_route;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;

#[tokio::test]
async fn reads_the_bearer_token_of_a_request() {
    let fixture = TlsFixture::generate();
    let server =
        RunningFixtureServer::start(fixture.server_config.clone(), vec![echo_bearer_route()]).await;
    let client = fixture_client_builder(&fixture.certificate_authority)
        .build()
        .expect("the fixture client builds");

    let response = client
        .get(fixture.url(server.port(), "/"))
        .header(AUTHORIZATION, "Bearer mF_9.B5f-4.1JqM")
        .send()
        .await
        .expect("the fixture server answers");

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        response.text().await.expect("the body is text"),
        "mF_9.B5f-4.1JqM"
    );

    server.stop().await;
}

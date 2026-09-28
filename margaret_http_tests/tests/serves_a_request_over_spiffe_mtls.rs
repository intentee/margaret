use margaret_http_tests::echo_peer_route::echo_peer_route;
use margaret_http_tests::mtls_client_request::mtls_client_request;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;

#[tokio::test]
async fn serves_a_request_over_spiffe_mtls() {
    let fixture = MtlsFixture::new();
    let server =
        RunningFixtureServer::start(fixture.server_config.clone(), vec![echo_peer_route()]).await;

    let response = mtls_client_request(
        fixture.client_config.clone(),
        &fixture.server_name,
        server.address(),
    )
    .await;

    assert!(response.contains(" 200 "));
    assert!(response.contains("example.org/test-client"));

    server.stop().await;
}

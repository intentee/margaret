use margaret_http_tests::echo_peer_route::echo_peer_route;
use margaret_http_tests::mtls_client_request::mtls_client_request;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;

#[tokio::test]
async fn forbids_a_peer_without_a_spiffe_id() {
    let fixture = MtlsFixture::default();
    let server =
        RunningFixtureServer::start(fixture.server_config.clone(), vec![echo_peer_route()]).await;

    let response = mtls_client_request(
        fixture.client_config_without_spiffe_id.clone(),
        &fixture.server_name,
        server.address(),
    )
    .await;

    assert!(response.contains(" 403 "));

    server.stop().await;
}

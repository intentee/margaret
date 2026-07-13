use margaret_http_tests::mtls_client_request::mtls_client_request;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_http_tests::running_mtls_server::RunningMtlsServer;

#[tokio::test]
async fn serves_a_request_over_spiffe_mtls() {
    let fixture = MtlsFixture::new();
    let server = RunningMtlsServer::start(fixture.server_config.clone()).await;

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

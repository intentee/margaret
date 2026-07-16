use std::sync::Arc;

use margaret_http::transport_config::TransportConfig;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_identity::margaret::container::build::build;
use margaret_identity::margaret::http::server_internal::server_internal;
use margaret_identity::margaret::routes::Routes;
use margaret_identity_tests::mtls_post::mtls_post;
use margaret_identity_tests::running_server::RunningServer;

#[tokio::test(flavor = "multi_thread")]
async fn internal_server_forbids_a_peer_without_a_spiffe_id() {
    let fixture = MtlsFixture::new();
    let container = build();
    let routes = Arc::new(Routes::from_origins(
        Arc::from("https://internal.example.org"),
        Arc::from("https://public.example.org"),
    ));
    let server_routes = server_internal(&container, &routes)
        .await
        .expect("the internal routes build");
    let server = RunningServer::start(
        server_routes,
        TransportConfig::MutualTls {
            server_config: fixture.server_config.clone(),
        },
    )
    .await;

    let response = mtls_post(
        fixture.client_config_without_spiffe_id.clone(),
        &fixture.server_name,
        server.address(),
        "/access_token/mint",
        "{\"refresh_token\":\"unused\"}",
    )
    .await;
    server.stop().await;

    assert!(response.contains("403 Forbidden"));
}

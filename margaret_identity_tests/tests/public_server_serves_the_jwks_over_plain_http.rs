use std::sync::Arc;

use margaret_http::transport_config::TransportConfig;
use margaret_identity::margaret::container::build::build;
use margaret_identity::margaret::http::server_public::server_public;
use margaret_identity::margaret::routes::Routes;
use margaret_identity_tests::plain_get::plain_get;
use margaret_identity_tests::running_server::RunningServer;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;

#[tokio::test(flavor = "multi_thread")]
async fn public_server_serves_the_jwks_over_plain_http() {
    let container = build();
    let store = container.stores_signing_key_store_signing_key_store().await;
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret is generated");
    store.holder().set(Some(Arc::new(secret)));
    let routes = Arc::new(Routes::from_origins(
        Arc::from("https://internal.example.org"),
        Arc::from("https://public.example.org"),
    ));
    let server_routes = server_public(&container, &routes)
        .await
        .expect("the public routes build");
    let server = RunningServer::start(server_routes, TransportConfig::Plain).await;

    let response = plain_get(server.address(), "/.well-known/jwks.json").await;
    server.stop().await;

    assert!(response.contains("200 OK"));
    assert!(response.contains("P-256"));
}

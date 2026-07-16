use std::sync::Arc;

use margaret_http::transport_config::TransportConfig;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_identity::margaret::container::build::build;
use margaret_identity::margaret::http::server_internal::server_internal;
use margaret_identity::margaret::routes::Routes;
use margaret_identity_tests::mtls_post::mtls_post;
use margaret_identity_tests::running_server::RunningServer;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;

#[tokio::test(flavor = "multi_thread")]
async fn internal_server_mints_tokens_over_mtls() {
    let fixture = MtlsFixture::new();
    let container = build();
    let store = container.stores_signing_key_store_signing_key_store().await;
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret is generated");
    let refresh_token =
        sign_refresh_token(&secret.current.signing, &refresh_claims(32_503_680_000)).await;
    store.holder().set(Some(Arc::new(secret)));
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

    let body = format!("{{\"refresh_token\":\"{refresh_token}\"}}");
    let response = mtls_post(
        fixture.client_config.clone(),
        &fixture.server_name,
        server.address(),
        "/access_token/mint",
        &body,
    )
    .await;
    server.stop().await;

    assert!(response.contains("200 OK"));
    assert!(response.contains("access_token"));
}

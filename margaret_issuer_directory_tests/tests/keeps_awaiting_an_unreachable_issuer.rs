use serde_json::json;
use tokio::net::TcpListener;

use margaret_issuer_directory_tests::first_poll::first_poll;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_oidc_issuer::localhost_oidc_issuer;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_an_unreachable_issuer() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(200, &json!({})),
        key_set: json_handler(200, &json!({ "keys": [] })),
    })
    .await;
    let closed = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a port binds")
        .local_addr()
        .expect("the bound port reports its address");

    assert!(matches!(
        first_poll(
            &localhost_oidc_issuer(),
            issuer.request_client_resolving_to(closed)
        )
        .await,
        KeySetRefresh::Refreshed(KeySetHolding::Awaiting)
    ));

    issuer.stop().await;
}

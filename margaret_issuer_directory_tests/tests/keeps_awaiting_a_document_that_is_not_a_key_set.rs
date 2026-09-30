use serde_json::json;

use margaret_issuer_directory_tests::first_poll::first_poll;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_jwks_endpoint::localhost_jwks_endpoint;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_a_document_that_is_not_a_key_set() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(404, &json!({})),
        key_set: json_handler(200, &json!({ "keys": "none" })),
    })
    .await;

    assert!(matches!(
        first_poll(&localhost_jwks_endpoint(), issuer.request_client()).await,
        KeySetRefresh::Refreshed(KeySetHolding::Awaiting)
    ));

    issuer.stop().await;
}

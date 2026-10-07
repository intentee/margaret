use serde_json::json;

use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_jwks_endpoint::localhost_jwks_endpoint;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_an_issuer_that_answers_the_key_set_request_with_an_error_status() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(404, &json!({})),
        key_set: json_handler(503, &json!({})),
    })
    .await;

    assert!(matches!(
        localhost_jwks_endpoint()
            .first_poll(issuer.request_client())
            .await,
        KeySetRefresh::Refreshed(KeySetHolding::Awaiting)
    ));

    issuer.stop().await;
}

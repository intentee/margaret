use std::sync::Arc;

use serde_json::json;

use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_directory_tests::first_poll::first_poll;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_jwks_endpoint::localhost_jwks_endpoint;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_response_max_bytes::ISSUER_RESPONSE_MAX_BYTES;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_an_oversized_key_set_document() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(404, &json!({})),
        key_set: Arc::new(StaticHandler {
            body: vec![b' '; ISSUER_RESPONSE_MAX_BYTES + 1],
            content_type: "application/json",
            status: 200,
        }),
    })
    .await;

    assert!(matches!(
        first_poll(&localhost_jwks_endpoint(), issuer.request_client()).await,
        KeySetRefresh::Refreshed(KeySetHolding::Awaiting)
    ));

    issuer.stop().await;
}

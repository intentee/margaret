use std::sync::Arc;

use serde_json::json;

use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_directory_tests::first_poll::first_poll;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_oidc_issuer::localhost_oidc_issuer;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_response_max_bytes::ISSUER_RESPONSE_MAX_BYTES;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_an_oversized_discovery_document() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: Arc::new(StaticHandler {
            body: vec![b' '; ISSUER_RESPONSE_MAX_BYTES + 1],
            content_type: "application/json",
            status: 200,
        }),
        key_set: json_handler(200, &json!({ "keys": [] })),
    })
    .await;

    assert!(matches!(
        first_poll(&localhost_oidc_issuer(), issuer.request_client()).await,
        KeySetRefresh::Refreshed(KeySetHolding::Awaiting)
    ));

    issuer.stop().await;
}

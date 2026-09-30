use std::sync::Arc;

use serde_json::json;

use margaret_http_tests::hanging_handler::HangingHandler;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_oidc_issuer::localhost_oidc_issuer;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;

#[tokio::test(flavor = "multi_thread")]
async fn stops_polling_when_cancelled_during_a_hung_discovery() {
    let hanging = Arc::new(HangingHandler::default());
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: hanging.clone(),
        key_set: json_handler(404, &json!({})),
    })
    .await;
    let trusted_issuer = localhost_oidc_issuer();
    let snapshot = trusted_issuer.key_set.snapshot();
    let directory =
        PolledDirectory::start(vec![Arc::clone(&trusted_issuer)], issuer.request_client());

    hanging.request_received.cancelled().await;
    directory.stop().await;

    assert!(matches!(
        trusted_issuer.key_set.refreshed_since(&snapshot).await,
        KeySetRefresh::PollingStopped
    ));

    hanging.release.cancel();
    issuer.stop().await;
}

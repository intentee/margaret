use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_http_tests::hanging_handler::HangingHandler;
use margaret_oidc_client::oidc_client::OidcClient;
use margaret_oidc_client_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use margaret_oidc_client_tests::running_fixture_issuer::RunningFixtureIssuer;

#[tokio::test(flavor = "multi_thread")]
async fn oidc_client_returns_promptly_when_cancelled_during_a_hung_discovery() {
    let hanging = Arc::new(HangingHandler::default());
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: hanging.clone(),
        key_set: hanging.clone(),
    })
    .await;
    let client = OidcClient::create(Arc::new(localhost_trust()));
    let cancellation_token = CancellationToken::new();
    let poll_token = cancellation_token.clone();
    let client_builder = issuer.client_builder();
    let poll_task = tokio::spawn(async move {
        client
            .run_with_client_builder(client_builder, poll_token)
            .await
    });

    hanging.request_received.cancelled().await;
    cancellation_token.cancel();

    poll_task
        .await
        .expect("the poll task joins")
        .expect("the oidc client shuts down cleanly");
    issuer.stop().await;
}

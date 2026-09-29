use std::sync::Arc;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_http_tests::hanging_handler::HangingHandler;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_key_set_poll::key_set_poll_fetch_timeout::KEY_SET_POLL_FETCH_TIMEOUT;
use margaret_key_set_poll::key_set_poll_service::KeySetPollService;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_key_set_poll_tests::first_tick_context::first_tick_context;
use margaret_oidc_client::discovery_key_set_locator::DiscoveryKeySetLocator;
use margaret_oidc_client_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use margaret_oidc_client_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;

#[tokio::test(start_paused = true)]
async fn key_set_poll_service_gives_a_hung_discovery_the_fetch_timeout() {
    let hanging = Arc::new(HangingHandler::default());
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: hanging.clone(),
        key_set: hanging.clone(),
    })
    .await;
    let token_trust = localhost_trust();
    let mut service = KeySetPollService {
        issuer_document_client: IssuerDocumentClient::build(issuer.client_builder())
            .expect("the issuer document client builds"),
        locator: DiscoveryKeySetLocator {
            discovery_url: oidc_discovery_url(&token_trust.issuer),
            token_trust: Arc::new(token_trust),
        },
        verification_key_set_holder: VerificationKeySetHolder::default(),
    };
    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), KEY_SET_POLL_FETCH_TIMEOUT);

    hanging.release.cancel();
    issuer.stop().await;
}

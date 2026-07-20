use std::time::Duration;

use reqwest::Client;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::jwk_public_set_holder::JwkPublicSetHolder;
use margaret_jwks_client::jwk_public_set_poll_service::JwkPublicSetPollService;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_client_tests::unreachable_endpoint::unreachable_endpoint;

#[tokio::test(start_paused = true)]
async fn jwk_public_set_poll_service_does_not_wait_before_the_first_key_is_fetched() {
    let mut service = JwkPublicSetPollService {
        endpoint: unreachable_endpoint(),
        http_client: Client::new(),
        jwk_public_set_holder: JwkPublicSetHolder::default(),
    };

    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), Duration::ZERO);
}

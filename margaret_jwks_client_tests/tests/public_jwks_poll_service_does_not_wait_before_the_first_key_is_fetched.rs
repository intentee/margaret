use std::time::Duration;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::default_http_client::default_http_client;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_client_tests::unreachable_endpoint::unreachable_endpoint;

#[tokio::test(start_paused = true)]
async fn public_jwks_poll_service_does_not_wait_before_the_first_key_is_fetched() {
    let mut service = PublicJwksPollService::new(
        PublicJwksHolder::default(),
        unreachable_endpoint(),
        default_http_client(),
    );

    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), Duration::ZERO);
}

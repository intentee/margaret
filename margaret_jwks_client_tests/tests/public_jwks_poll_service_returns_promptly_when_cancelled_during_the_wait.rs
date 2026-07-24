use std::sync::Arc;

use reqwest::Client;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_client_tests::unreachable_endpoint::unreachable_endpoint;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;

#[tokio::test(start_paused = true)]
async fn public_jwks_poll_service_returns_promptly_when_cancelled_during_the_wait() {
    let known_good = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let public_jwks_holder = PublicJwksHolder::default();

    public_jwks_holder.set(Some(Arc::new(PublicJwks::from(known_good))));

    let mut service = PublicJwksPollService {
        endpoint_provider: unreachable_endpoint(),
        http_client: Client::new(),
        public_jwks_holder,
    };

    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    let started_at = Instant::now();

    service
        .handle_tick(cancellation_token, first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert!(started_at.elapsed() < JWKS_POLL_INTERVAL_AFTER_READY);
}

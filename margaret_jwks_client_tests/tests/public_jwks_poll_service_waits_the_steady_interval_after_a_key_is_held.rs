use std::sync::Arc;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_client_tests::http_client_without_trust_store::http_client_without_trust_store;
use margaret_jwks_client_tests::unreachable_endpoint::unreachable_endpoint;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;

#[tokio::test(start_paused = true)]
async fn public_jwks_poll_service_waits_the_steady_interval_after_a_key_is_held() {
    let known_good = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let public_jwks_holder = PublicJwksHolder::default();

    public_jwks_holder.set(Some(Arc::new(PublicJwks::from(known_good))));

    let mut service = PublicJwksPollService {
        endpoint_provider: unreachable_endpoint(),
        http_client: http_client_without_trust_store(),
        public_jwks_holder,
    };

    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), JWKS_POLL_INTERVAL_AFTER_READY);
}

use std::sync::Arc;

use reqwest::Client;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::jwk_public_set_holder::JwkPublicSetHolder;
use margaret_jwks_client::jwk_public_set_poll_service::JwkPublicSetPollService;
use margaret_jwks_client::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_client_tests::unreachable_issuer_url::unreachable_issuer_url;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;

#[tokio::test(start_paused = true)]
async fn jwk_public_set_poll_service_waits_the_steady_interval_after_a_key_is_held() {
    let known_good = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let jwk_public_set_holder = JwkPublicSetHolder::default();

    jwk_public_set_holder.set(Some(Arc::new(JwkPublicSet::from(known_good))));

    let mut service = JwkPublicSetPollService {
        http_client: Client::new(),
        jwk_public_set_holder,
        jwks_url: unreachable_issuer_url(),
    };

    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), JWKS_POLL_INTERVAL_AFTER_READY);
}

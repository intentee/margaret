use std::sync::Arc;

use reqwest::Client;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::jwk_public_set_holder::JwkPublicSetHolder;
use margaret_jwks_client::jwk_public_set_poll_service::JwkPublicSetPollService;
use margaret_jwks_client_tests::failing_endpoint::FailingEndpoint;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;

#[tokio::test(start_paused = true)]
async fn jwk_public_set_poll_service_retains_the_last_set_when_the_endpoint_fails() {
    let known_good = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let known_good_kid = known_good.current.public.kid.clone();
    let jwk_public_set_holder = JwkPublicSetHolder::default();

    jwk_public_set_holder.set(Some(Arc::new(JwkPublicSet::from(known_good))));

    let mut service = JwkPublicSetPollService {
        endpoint: Arc::new(FailingEndpoint),
        http_client: Client::new(),
        jwk_public_set_holder: jwk_public_set_holder.clone(),
    };

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed endpoint resolution never fails the service");

    let retained = jwk_public_set_holder
        .get()
        .expect("the last known good document survives a failed endpoint resolution");

    assert!(retained.find_by_kid(&known_good_kid).is_some());
}

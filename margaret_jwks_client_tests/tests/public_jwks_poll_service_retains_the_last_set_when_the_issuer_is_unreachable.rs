use std::sync::Arc;

use reqwest::Client;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_client_tests::unreachable_endpoint::unreachable_endpoint;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;

#[tokio::test(start_paused = true)]
async fn public_jwks_poll_service_retains_the_last_set_when_the_issuer_is_unreachable() {
    let known_good = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let known_good_kid = known_good.current.public.kid.clone();
    let public_jwks_holder = PublicJwksHolder::default();

    public_jwks_holder.set(Some(Arc::new(PublicJwks::from(known_good))));

    let mut service = PublicJwksPollService {
        endpoint_provider: unreachable_endpoint(),
        http_client: Client::new(),
        public_jwks_holder: public_jwks_holder.clone(),
    };

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    let retained = public_jwks_holder
        .get()
        .expect("the last known good document survives a failed poll");

    assert!(
        retained
            .find_by_kid(&known_good_kid)
            .expect("published key ids are unique")
            .is_some()
    );
}

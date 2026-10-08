use std::sync::Arc;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::ServiceBundle as _;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test(start_paused = true)]
async fn jwks_roller_server_bundle_rotates_its_secret_one_interval_after_it_starts() {
    let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: Arc::new(FixtureSigningKeys::empty()),
    })
    .await
    .expect("the first secret is rolled and published");
    let jwks_secret_holder = bundle.jwks_secret_holder();
    let initial = jwks_secret_holder.get();
    let mut secret_subscription = jwks_secret_holder.subscribe();
    let mut services = bundle
        .services()
        .await
        .expect("the bundle exposes services");
    let service = services.pop().expect("the bundle exposes the roll service");

    assert!(services.is_empty());

    let started_at = Instant::now();
    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { service.run(cancellation_token_for_service).await });

    secret_subscription.changed().await;

    let rotated_at = Instant::now();

    cancellation_token.cancel();
    service_task
        .await
        .expect("the service task joins")
        .expect("the roll service shuts down cleanly");

    assert!(rotated_at - started_at >= JWKS_ROLL_INTERVAL);
    assert!(
        secret_subscription
            .read_current()
            .previous()
            .is_retired_key(initial.current().kid())
    );
}

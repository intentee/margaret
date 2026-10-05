use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::ServiceBundle as _;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;

#[tokio::test(start_paused = true)]
async fn jwks_roller_server_bundle_republishes_its_document_when_it_rotates() {
    let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: Arc::new(MemoryJwksSecretStorage),
    })
    .expect("the first secret is rolled and published");
    let document_holder = bundle.jwks_document_holder();
    let initial = document_holder.get();
    let mut document_subscription = document_holder.subscribe();
    let mut services = bundle
        .services()
        .await
        .expect("the bundle exposes services");
    let service = services.pop().expect("the bundle exposes the roll service");
    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { service.run(cancellation_token_for_service).await });

    document_subscription.changed().await;
    cancellation_token.cancel();
    service_task
        .await
        .expect("the service task joins")
        .expect("the roll service shuts down cleanly");

    assert_ne!(document_subscription.read_current(), initial);
}

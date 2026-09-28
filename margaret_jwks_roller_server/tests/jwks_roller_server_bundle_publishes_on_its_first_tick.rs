use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::ServiceBundle as _;

use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;

#[tokio::test(start_paused = true)]
async fn jwks_roller_server_bundle_publishes_on_its_first_tick() {
    let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        storage: Arc::new(MemoryJwksSecretStorage),
    });
    let jwks_secret_holder = bundle.jwks_secret_holder();

    assert!(jwks_secret_holder.get().is_none());

    let mut services = bundle
        .services()
        .await
        .expect("the bundle exposes services");
    let service = services.pop().expect("the bundle exposes the roll service");

    assert!(services.is_empty());

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { service.run(cancellation_token_for_service).await });

    let mut secret_subscription = jwks_secret_holder.subscribe();

    while secret_subscription.read_current().is_none() {
        secret_subscription.changed().await;
    }

    cancellation_token.cancel();
    service_task
        .await
        .expect("the service task joins")
        .expect("the roll service shuts down cleanly");

    let published = jwks_secret_holder.get().expect("the first tick published");

    assert_ne!(published.next().kid(), published.current().kid());
}

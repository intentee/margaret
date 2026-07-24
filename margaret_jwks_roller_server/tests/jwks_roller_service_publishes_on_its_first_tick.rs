use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::jwks_document_holder::JwksDocumentHolder;
use margaret_jwks_roller_server::jwks_roller_service::JwksRollerService;

#[tokio::test(start_paused = true)]
async fn jwks_roller_service_publishes_on_its_first_tick() {
    let jwks_secret_holder = JwksSecretHolder::default();

    assert!(jwks_secret_holder.get().is_none());

    let service = JwksRollerService::new(
        JwksDocumentHolder::default(),
        jwks_secret_holder.clone(),
        Arc::new(MemoryJwksSecretStorage),
    );

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { Box::new(service).run(cancellation_token_for_service).await });

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

    assert_ne!(published.next.public.kid, published.current.public.kid);
}

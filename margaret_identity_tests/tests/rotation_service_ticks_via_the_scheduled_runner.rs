use std::sync::Arc;

use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_identity::tickers::jwks_rotation_service::JwksRotationService;
use margaret_service::scheduled_argument_runner::ScheduledArgumentRunner;
use tempfile::tempdir;

#[tokio::test]
async fn rotation_service_ticks_via_the_scheduled_runner() {
    let directory = tempdir().expect("a temporary directory is created");
    let store = Arc::new(SigningKeyStore::create());
    let service = JwksRotationService::create(store.clone());

    service
        .run_scheduled_tick(directory.path().join("jwks.json"))
        .await
        .expect("the scheduled tick seeds the store");

    assert!(store.current().is_some());
}

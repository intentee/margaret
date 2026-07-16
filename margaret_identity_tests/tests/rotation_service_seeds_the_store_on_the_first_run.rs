use std::sync::Arc;

use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_identity::tickers::jwks_rotation_service::JwksRotationService;
use tempfile::tempdir;

#[tokio::test]
async fn rotation_service_seeds_the_store_on_the_first_run() {
    let directory = tempdir().expect("a temporary directory is created");
    let store = Arc::new(SigningKeyStore::create());
    let service = JwksRotationService::create(store.clone());

    service
        .run(directory.path().join("jwks.json"))
        .await
        .expect("the first run seeds the store");

    assert!(store.current().is_some());
}

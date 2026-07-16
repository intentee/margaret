use std::sync::Arc;

use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_identity::tickers::jwks_rotation_service::JwksRotationService;
use tempfile::tempdir;

#[tokio::test]
async fn rotation_service_reports_a_maintenance_error_for_an_unwritable_path() {
    let directory = tempdir().expect("a temporary directory is created");
    let store = Arc::new(SigningKeyStore::create());
    let service = JwksRotationService::create(store);

    let error = service
        .run(directory.path().join("missing").join("jwks.json"))
        .await
        .expect_err("an unwritable path fails to persist");

    assert!(error.to_string().contains("failed to maintain"));
}

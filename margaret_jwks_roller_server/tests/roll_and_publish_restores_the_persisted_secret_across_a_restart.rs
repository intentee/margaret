use std::sync::Arc;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;

#[test]
fn roll_and_publish_restores_the_persisted_secret_across_a_restart() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");

    let before_restart = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        storage: Arc::new(FileJwksSecretStorage::new(path.clone())),
    });

    before_restart
        .roll_and_publish()
        .expect("the first start persists a secret");

    let persisted_kid = before_restart
        .jwks_secret_holder()
        .get()
        .expect("the first start seeds the holder")
        .current()
        .kid()
        .clone();

    let after_restart = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        storage: Arc::new(FileJwksSecretStorage::new(path)),
    });

    after_restart
        .roll_and_publish()
        .expect("the restarted server restores the secret");

    let restored_kid = after_restart
        .jwks_secret_holder()
        .get()
        .expect("the restarted server seeds the holder")
        .current()
        .kid()
        .clone();

    assert_eq!(restored_kid, persisted_kid);
}

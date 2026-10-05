use std::path::Path;
use std::sync::Arc;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use margaret_jws_verification::key_id::KeyId;

fn started_kid(path: &Path) -> KeyId {
    JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: Arc::new(FileJwksSecretStorage::new(path.to_path_buf())),
    })
    .expect("the server starts with a secret")
    .jwks_secret_holder()
    .get()
    .current()
    .kid()
    .clone()
}

#[test]
fn jwks_roller_server_bundle_restores_the_persisted_secret_across_a_restart() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");

    assert_eq!(started_kid(&path), started_kid(&path));
}

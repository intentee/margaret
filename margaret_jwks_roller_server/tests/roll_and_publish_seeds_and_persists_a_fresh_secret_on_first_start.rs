use std::sync::Arc;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;

#[test]
fn roll_and_publish_seeds_and_persists_a_fresh_secret_on_first_start() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");

    assert!(!path.exists());

    let first_start = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: Arc::new(FileJwksSecretStorage::new(path.clone())),
    });

    first_start
        .roll_and_publish()
        .expect("the first start seeds a fresh secret");

    let seeded = first_start
        .jwks_secret_holder()
        .get()
        .expect("the first start seeds the holder");

    assert!(matches!(seeded.previous(), PreviousKey::Absent));
    assert!(path.exists());
}

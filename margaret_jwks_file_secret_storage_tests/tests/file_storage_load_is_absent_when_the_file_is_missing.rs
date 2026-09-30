use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;

#[test]
fn file_storage_load_is_absent_when_the_file_is_missing() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let storage = FileJwksSecretStorage::new(directory.path().join("jwks.json"));

    let loaded = storage
        .load(&FixtureRsaSigningKeys::default())
        .expect("a missing secret file loads as absent");

    assert!(matches!(loaded, LoadedSecret::Absent));
}

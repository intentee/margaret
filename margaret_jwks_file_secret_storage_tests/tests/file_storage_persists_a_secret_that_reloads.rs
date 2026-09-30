use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage_tests::sample_secret::sample_secret;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;

#[test]
fn file_storage_persists_a_secret_that_reloads() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let storage = FileJwksSecretStorage::new(directory.path().join("jwks.json"));
    let secret = sample_secret();

    storage
        .persist(&secret)
        .expect("the secret can be persisted");

    let LoadedSecret::Present(reloaded) = storage
        .load(&FixtureRsaSigningKeys::default())
        .expect("the persisted secret can be reloaded")
    else {
        panic!("the persisted secret must reload as present");
    };

    assert_eq!(reloaded.current().kid(), secret.current().kid());
    assert_eq!(reloaded.next().kid(), secret.next().kid());
    assert!(matches!(reloaded.previous(), PreviousKey::Absent));
}

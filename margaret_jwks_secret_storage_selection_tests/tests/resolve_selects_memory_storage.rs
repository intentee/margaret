use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;
use margaret_jwks_secret_storage_selection::resolve_jwks_secret_storage::resolve_jwks_secret_storage;

#[test]
fn resolve_selects_memory_storage() {
    let storage = resolve_jwks_secret_storage(JwksSecretStorageUri::Memory);

    let loaded = storage
        .load(&FixtureRsaSigningKeys::default())
        .expect("the memory storage loads");

    assert!(matches!(loaded, LoadedSecret::Absent));
}

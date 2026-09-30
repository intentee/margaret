use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;
use margaret_jwks_secret_storage_selection::resolve_jwks_secret_storage::resolve_jwks_secret_storage;

#[test]
fn resolve_selects_file_storage() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");
    let storage = resolve_jwks_secret_storage(JwksSecretStorageUri::File { path });
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh jwks secret");

    storage
        .persist(&secret)
        .expect("the file storage persists the secret");

    let LoadedSecret::Present(reloaded) = storage
        .load(&FixtureRsaSigningKeys::default())
        .expect("the file storage loads")
    else {
        panic!("the file storage must reload the persisted secret");
    };

    assert_eq!(reloaded.current().kid(), secret.current().kid());
}

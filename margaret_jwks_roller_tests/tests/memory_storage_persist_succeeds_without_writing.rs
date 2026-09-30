use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

#[test]
fn memory_storage_persist_succeeds_without_writing() {
    let storage = MemoryJwksSecretStorage;
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");

    assert!(storage.persist(&secret).is_ok());
}

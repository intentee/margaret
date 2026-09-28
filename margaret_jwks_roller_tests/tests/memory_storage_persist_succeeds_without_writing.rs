use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

#[test]
fn memory_storage_persist_succeeds_without_writing() {
    let storage = MemoryJwksSecretStorage;
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");

    assert!(storage.persist(&secret).is_ok());
}

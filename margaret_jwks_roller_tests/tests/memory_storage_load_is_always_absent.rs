use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

#[test]
fn memory_storage_load_is_always_absent() {
    let storage = MemoryJwksSecretStorage;

    assert!(matches!(storage.load(), Ok(LoadedSecret::Absent)));
}

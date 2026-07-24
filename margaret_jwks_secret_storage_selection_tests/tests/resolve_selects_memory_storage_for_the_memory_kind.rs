use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_kind::JwksSecretStorageKind;
use margaret_jwks_secret_storage_selection::resolve_jwks_secret_storage::resolve_jwks_secret_storage;

#[test]
fn resolve_selects_memory_storage_for_the_memory_kind() {
    let storage = resolve_jwks_secret_storage(JwksSecretStorageKind::Memory, None)
        .expect("the memory storage resolves");

    let loaded = storage.load().expect("the memory storage loads");

    assert!(matches!(loaded, LoadedSecret::Absent));
}

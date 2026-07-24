use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_kind::JwksSecretStorageKind;
use margaret_jwks_secret_storage_selection::resolve_jwks_secret_storage::resolve_jwks_secret_storage;

#[test]
fn resolve_selects_file_storage_for_the_file_kind() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");
    let storage = resolve_jwks_secret_storage(JwksSecretStorageKind::File, Some(path))
        .expect("the file storage resolves");
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh jwks secret");

    storage
        .persist(&secret)
        .expect("the file storage persists the secret");

    let LoadedSecret::Present(reloaded) = storage.load().expect("the file storage loads") else {
        panic!("the file storage must reload the persisted secret");
    };

    assert_eq!(reloaded.current.public.kid, secret.current.public.kid);
}

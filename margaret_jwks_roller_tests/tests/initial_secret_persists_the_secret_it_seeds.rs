use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_roller_tests::stored_jwks_secret_storage::StoredJwksSecretStorage;

#[test]
fn initial_secret_persists_the_secret_it_seeds() {
    let storage = StoredJwksSecretStorage::empty();

    let seeded = initial_secret(
        &storage,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .expect("a fresh secret is seeded");
    let stored = storage
        .load(&FixtureRsaSigningKeys::default())
        .expect("the backend can be read");

    match stored {
        LoadedSecret::Absent => panic!("the seeded secret must be persisted"),
        LoadedSecret::Present(secret) => {
            assert_eq!(secret.current().kid(), seeded.current().kid());
            assert_eq!(secret.next().kid(), seeded.next().kid());
        }
    }
}

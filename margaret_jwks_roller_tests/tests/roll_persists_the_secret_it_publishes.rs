use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller_tests::stored_jwks_secret_storage::StoredJwksSecretStorage;

#[test]
fn roll_persists_the_secret_it_publishes() {
    let storage = StoredJwksSecretStorage::empty();
    let holder = JwksSecretHolder::default();

    roll(
        &storage,
        &holder,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .expect("the roll seeds a fresh secret");

    let published = holder.get().expect("the holder is seeded");
    let stored = storage
        .load(&FixtureRsaSigningKeys::default())
        .expect("the backend can be read");

    match stored {
        LoadedSecret::Absent => panic!("the roll must persist the secret it publishes"),
        LoadedSecret::Present(secret) => {
            assert_eq!(secret.current().kid(), published.current().kid());
            assert_eq!(secret.next().kid(), published.next().kid());
        }
    }
}

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller_tests::stored_jwks_secret_storage::StoredJwksSecretStorage;

#[test]
fn roll_loads_the_persisted_secret_when_the_holder_is_empty() {
    let persisted = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let persisted_kid = persisted.current().kid().clone();
    let storage = StoredJwksSecretStorage::seeded(persisted);
    let holder = JwksSecretHolder::default();

    roll(
        &storage,
        &holder,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .expect("the roll loads the persisted secret");

    let loaded = holder.get().expect("the holder is seeded");

    assert_eq!(loaded.current().kid(), &persisted_kid);
}

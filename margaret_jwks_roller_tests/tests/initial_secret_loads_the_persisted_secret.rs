use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller_tests::stored_jwks_secret_storage::StoredJwksSecretStorage;

#[test]
fn initial_secret_loads_the_persisted_secret() {
    let persisted = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let persisted_kid = persisted.current().kid().clone();

    let loaded = initial_secret(
        &StoredJwksSecretStorage::seeded(persisted),
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .expect("the persisted secret is loaded");

    assert_eq!(loaded.current().kid(), &persisted_kid);
}

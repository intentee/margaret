use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn initial_secret_loads_the_persisted_secret() {
    let persisted = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let storage = FixtureSigningKeys::storing(&persisted);

    let loaded = initial_secret(
        &storage,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    .expect("the persisted secret is loaded");

    assert_eq!(loaded.current().kid(), persisted.current().kid());
    assert_eq!(storage.stored_documents().await, 1);
}

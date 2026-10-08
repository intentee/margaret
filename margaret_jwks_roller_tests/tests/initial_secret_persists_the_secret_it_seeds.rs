use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn initial_secret_persists_the_secret_it_seeds() {
    let storage = FixtureSigningKeys::empty();

    let seeded = initial_secret(
        &storage,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    .expect("a fresh secret is seeded");
    let restored = initial_secret(
        &storage,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    .expect("the seeded secret is restored");

    assert_eq!(restored.current().kid(), seeded.current().kid());
    assert_eq!(restored.next().kid(), seeded.next().kid());
}

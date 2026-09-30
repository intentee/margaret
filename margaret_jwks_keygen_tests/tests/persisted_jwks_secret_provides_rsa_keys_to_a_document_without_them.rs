use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn persisted_jwks_secret_provides_rsa_keys_to_a_document_without_them() -> Result<()> {
    let rsa_keys = FixtureRsaSigningKeys::default();
    let secret = JwksSecret::fresh(SigningCurve::P256, &rsa_keys)?;
    let mut document = serde_json::to_value(PersistedJwksSecret::from_secret(&secret))?;

    document
        .as_object_mut()
        .expect("the persisted secret is an object")
        .remove("rsa");

    let restored =
        serde_json::from_value::<PersistedJwksSecret>(document)?.into_secret(&rsa_keys)?;

    assert_eq!(restored.current().kid(), secret.current().kid());
    assert_ne!(restored.rsa().current().kid(), secret.rsa().current().kid());
    assert!(matches!(restored.rsa().previous(), PreviousKey::Absent));

    Ok(())
}

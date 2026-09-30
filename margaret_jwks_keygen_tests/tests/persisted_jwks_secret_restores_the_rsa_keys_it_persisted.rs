use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn persisted_jwks_secret_restores_the_rsa_keys_it_persisted() -> Result<()> {
    let rsa_keys = FixtureRsaSigningKeys::default();
    let rotated = JwksSecret::fresh(SigningCurve::P256, &rsa_keys)?.rotate(&rsa_keys)?;
    let restored = serde_json::from_slice::<PersistedJwksSecret>(&serde_json::to_vec(
        &PersistedJwksSecret::from_secret(&rotated),
    )?)?
    .into_secret(&rsa_keys)?;
    let PreviousKey::Retired(retired) = rotated.rsa().previous() else {
        panic!("the rotated ring retires its current key");
    };
    let PreviousKey::Retired(restored_retired) = restored.rsa().previous() else {
        panic!("the restored ring keeps its retired key");
    };

    assert_eq!(
        restored.rsa().current().kid(),
        rotated.rsa().current().kid()
    );
    assert_eq!(
        restored.rsa().current().signing_key().pkcs8(),
        rotated.rsa().current().signing_key().pkcs8()
    );
    assert_eq!(restored.rsa().next().kid(), rotated.rsa().next().kid());
    assert_eq!(restored_retired.kid(), retired.kid());
    assert_eq!(restored.public_jwks().keys(), rotated.public_jwks().keys());

    Ok(())
}

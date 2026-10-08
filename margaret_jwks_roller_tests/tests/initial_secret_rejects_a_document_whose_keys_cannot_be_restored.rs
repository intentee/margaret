use serde_json::Value;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

fn corrupt_every_pem(value: &mut Value) {
    match value {
        Value::Object(members) => {
            for (name, member) in members.iter_mut() {
                if name == "pem" {
                    *member = Value::String("not a pem document".to_string());
                } else {
                    corrupt_every_pem(member);
                }
            }
        }
        Value::Array(elements) => elements.iter_mut().for_each(corrupt_every_pem),
        Value::Bool(_) | Value::Null | Value::Number(_) | Value::String(_) => {}
    }
}

#[tokio::test]
async fn initial_secret_rejects_a_document_whose_keys_cannot_be_restored() {
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let mut document = serde_json::to_value(PersistedJwksSecret::from_secret(&secret))
        .expect("the secret serializes");

    corrupt_every_pem(&mut document);

    let Err(error) = initial_secret(
        &FixtureSigningKeys::holding(&document.to_string()),
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    else {
        panic!("the corrupted keys are not restored");
    };

    assert!(matches!(error, RollerError::DocumentRestore { .. }));
}

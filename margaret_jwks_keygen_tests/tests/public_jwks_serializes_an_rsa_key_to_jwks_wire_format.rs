use anyhow::Result;
use serde_json::to_value;

use margaret_jwks_keygen::jwk_public::JwkPublic;
use margaret_jwks_keygen::key_use::KeyUse;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::rsa_algorithm::RsaAlgorithm;
use margaret_jwks_keygen::rsa_jwk_public::RsaJwkPublic;
use margaret_jwks_keygen_tests::rsa_test_key_exponent::RSA_TEST_KEY_EXPONENT;
use margaret_jwks_keygen_tests::rsa_test_key_modulus::RSA_TEST_KEY_MODULUS;

#[test]
fn public_jwks_serializes_an_rsa_key_to_jwks_wire_format() -> Result<()> {
    let public_jwks = PublicJwks::new(vec![JwkPublic::Rsa(RsaJwkPublic {
        alg: RsaAlgorithm::Rs256,
        e: RSA_TEST_KEY_EXPONENT.to_string(),
        kid: "wire-kid".to_string(),
        n: RSA_TEST_KEY_MODULUS.to_string(),
        use_: KeyUse::Signature,
    })])?;

    let serialized = to_value(&public_jwks)?;
    let key = &serialized["keys"][0];

    assert_eq!(key["alg"], "RS256");
    assert_eq!(key["e"], RSA_TEST_KEY_EXPONENT);
    assert_eq!(key["kid"], "wire-kid");
    assert_eq!(key["kty"], "RSA");
    assert_eq!(key["n"], RSA_TEST_KEY_MODULUS);
    assert_eq!(key["use"], "sig");

    Ok(())
}

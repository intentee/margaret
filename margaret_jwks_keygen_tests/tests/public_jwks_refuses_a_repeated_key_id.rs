use margaret_jwks_keygen::jwk_public::JwkPublic;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::key_use::KeyUse;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::rsa_algorithm::RsaAlgorithm;
use margaret_jwks_keygen::rsa_jwk_public::RsaJwkPublic;
use margaret_jwks_keygen_tests::rsa_test_key_exponent::RSA_TEST_KEY_EXPONENT;
use margaret_jwks_keygen_tests::rsa_test_key_modulus::RSA_TEST_KEY_MODULUS;

fn shared_kid_key() -> JwkPublic {
    JwkPublic::Rsa(RsaJwkPublic {
        alg: RsaAlgorithm::Rs256,
        e: RSA_TEST_KEY_EXPONENT.to_string(),
        kid: "shared-kid".to_string(),
        n: RSA_TEST_KEY_MODULUS.to_string(),
        use_: KeyUse::Signature,
    })
}

#[test]
fn public_jwks_refuses_a_repeated_key_id() {
    assert!(matches!(
        PublicJwks::new(vec![shared_kid_key(), shared_kid_key()]),
        Err(JwksKeyError::DuplicateKeyId { kid }) if kid == "shared-kid"
    ));
}

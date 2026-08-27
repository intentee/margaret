use serde_json::from_str;

use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen_tests::rsa_test_key_exponent::RSA_TEST_KEY_EXPONENT;
use margaret_jwks_keygen_tests::rsa_test_key_modulus::RSA_TEST_KEY_MODULUS;

#[test]
fn public_jwks_refuses_a_repeated_key_id() {
    let key = format!(
        r#"{{"kty":"RSA","alg":"RS256","use":"sig","kid":"shared-kid","n":"{RSA_TEST_KEY_MODULUS}","e":"{RSA_TEST_KEY_EXPONENT}"}}"#
    );
    let document = format!(r#"{{"keys":[{key},{key}]}}"#);

    assert!(from_str::<PublicJwks>(&document).is_err());
}

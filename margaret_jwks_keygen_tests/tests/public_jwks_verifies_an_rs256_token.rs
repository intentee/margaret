use anyhow::Result;
use serde_json::from_str;

use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::rsa_test_key_exponent::RSA_TEST_KEY_EXPONENT;
use margaret_jwks_keygen_tests::rsa_test_key_modulus::RSA_TEST_KEY_MODULUS;
use margaret_jwks_keygen_tests::sign_rs256_token::sign_rs256_token;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn public_jwks_verifies_an_rs256_token() -> Result<()> {
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let token = sign_rs256_token("rsa-fixture-kid", &claims);
    let document = format!(
        r#"{{"keys":[{{"kty":"RSA","alg":"RS256","use":"sig","kid":"rsa-fixture-kid","n":"{RSA_TEST_KEY_MODULUS}","e":"{RSA_TEST_KEY_EXPONENT}"}}]}}"#
    );
    let public_jwks: PublicJwks = from_str(&document)?;
    let verified: TestClaims = public_jwks
        .verify(&token)?
        .verified()
        .expect("the token verifies");

    assert_eq!(verified, claims);

    Ok(())
}

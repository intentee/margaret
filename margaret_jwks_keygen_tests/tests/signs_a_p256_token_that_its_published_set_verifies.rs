use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;

#[test]
fn signs_a_p256_token_that_its_published_set_verifies() -> Result<()> {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(Curve::P256)?;
    let token = claims.signed_by(secret.current());
    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    assert!(matches!(
        key_set.verify(&token),
        JwsVerification::Verified(_)
    ));

    Ok(())
}

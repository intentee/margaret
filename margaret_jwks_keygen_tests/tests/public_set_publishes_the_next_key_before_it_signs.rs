use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;

#[test]
fn public_set_publishes_the_next_key_before_it_signs() -> Result<()> {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(Curve::P256)?;
    let token = claims.signed_by(secret.next());
    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    assert!(matches!(
        key_set.verify(&token),
        JwsVerification::Verified(_)
    ));

    Ok(())
}

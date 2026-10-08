use std::iter;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_keygen_tests::verified_token::verified_token;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::signing_key_retention::signing_key_retention;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn jwks_secret_verifies_refresh_tokens_of_a_key_retired_within_its_refresh_retention() {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = fresh_secret(SigningCurve::P256);
    let token = claims.signed_by(secret.current());
    let rolls_within_retention = signing_key_retention()
        .refresh
        .as_secs()
        .div_ceil(JWKS_ROLL_INTERVAL.as_secs());
    let retained = iter::successors(Some(secret), |secret| Some(rolled_secret(secret)))
        .nth(usize::try_from(rolls_within_retention).expect("the roll count fits"))
        .expect("the secret rolls");

    assert!(matches!(
        verified_token(retained.refresh_key_set(), &token),
        JwtVerification::Verified(verified) if verified.claims == claims
    ));
}

use std::iter;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_keygen_tests::verified_token::verified_token;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::signing_key_retention::signing_key_retention;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn jwks_secret_drops_a_key_retired_beyond_its_refresh_retention() {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = fresh_secret(SigningCurve::P256);
    let token = claims.signed_by(secret.current());
    let rolls_beyond_retention = signing_key_retention()
        .refresh
        .as_secs()
        .div_ceil(JWKS_ROLL_INTERVAL.as_secs())
        + 1;
    let lapsed = iter::successors(Some(secret), |secret| Some(rolled_secret(secret)))
        .nth(usize::try_from(rolls_beyond_retention).expect("the roll count fits"))
        .expect("the secret rolls");

    assert!(matches!(
        verified_token(lapsed.refresh_key_set(), &token),
        JwtVerification::Rejected(JwtRejection::Jws(JwsRejection::UnknownKeyId { .. }))
    ));
    assert_eq!(
        u64::try_from(lapsed.retired().len()).expect("the retired count fits"),
        rolls_beyond_retention - 1
    );
}

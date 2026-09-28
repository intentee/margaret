use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn jwks_secret_rejects_a_token_of_another_secret() -> Result<()> {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(Curve::P256)?;
    let token = claims.signed_by(JwksSecret::fresh(Curve::P256)?.current());

    assert!(matches!(
        secret.verify_jwt::<TestClaims>(&token, NumericDate::new(0)),
        JwksSecretVerificationResult::Rejected(JwtRejection::Jws(
            JwsRejection::UnknownKeyId { .. }
        ))
    ));

    Ok(())
}

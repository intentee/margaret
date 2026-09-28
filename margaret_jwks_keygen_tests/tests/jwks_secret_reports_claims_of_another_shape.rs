use anyhow::Result;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn jwks_secret_reports_claims_of_another_shape() -> Result<()> {
    let trust = fixture_trust();
    let secret = JwksSecret::fresh(Curve::P256)?;
    let token = secret
        .current()
        .sign_json(&json!("not the expected claims"), JwtType::AccessToken);

    assert!(matches!(
        secret.verify_jwt::<TestClaims>(
            &token,
            &JwtExpectation {
                audience: &trust.audience,
                issuer: &trust.issuer,
                token_type: TypeHeaderExpectation::Required(JwtType::AccessToken)
            },
            NumericDate::new(0)
        ),
        JwksSecretVerificationResult::Rejected(JwtRejection::Claims(
            ClaimsRejection::Malformed { .. }
        ))
    ));

    Ok(())
}

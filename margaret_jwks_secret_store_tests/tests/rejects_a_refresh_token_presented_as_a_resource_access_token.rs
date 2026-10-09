use uuid::Uuid;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_rejection::TypeRejection;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn rejects_a_refresh_token_presented_as_a_resource_access_token() {
    let store = rolled_store(fresh_secret(SigningCurve::P256)).await;
    let refresh_token = store.issue_refresh_token(Uuid::from_u128(7), unix_time(1_000));

    assert!(matches!(
        store.verify_resource_access_token(
            &refresh_token.signed_claims,
            &[fixture_issuance().audience],
            unix_time(1_000)
        ),
        JwtVerification::Rejected(JwtRejection::Type(TypeRejection::Mismatch {
            expected: JwtType::AccessToken,
            found: HeaderType::Supported(JwtType::Refresh),
        }))
    ));
}

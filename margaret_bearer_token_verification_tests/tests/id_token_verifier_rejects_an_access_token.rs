use margaret_bearer_token_verification::bearer_token_verification::BearerTokenVerification;
use margaret_bearer_token_verification_tests::id_token_verifier_holding::id_token_verifier_holding;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::type_rejection::TypeRejection;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn id_token_verifier_rejects_an_access_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let access_token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());
    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    assert!(matches!(
        id_token_verifier_holding(key_set).verify::<TestClaims>(
            &RequestAuthorization::parse(Some(&format!("Bearer {access_token}"))),
            NumericDate::new(1_700_000_000),
        ),
        BearerTokenVerification::Rejected(JwtRejection::Type(TypeRejection::Mismatch {
            expected: JwtType::Jwt,
            found: HeaderType::Supported(JwtType::AccessToken),
        }))
    ));
}

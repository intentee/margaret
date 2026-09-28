use uuid::Uuid;

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_jose_parameters::curve::Curve;
use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client_tests::verifier_holding::verifier_holding;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn public_jwks_verifier_rejects_an_access_token_at_its_exact_expiry() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let claims = AccessTokenClaims {
        sub: Uuid::from_u128(1),
    };
    let token = secret
        .current()
        .sign_json(&claims.to_payload(&RegisteredClaims {
            exp: NumericDate::new(1_700_000_000),
            iat: NumericDate::new(1_699_999_100),
            nbf: None,
        }));

    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    assert!(matches!(
        verifier_holding(key_set).verify::<AccessTokenClaims>(&token, unix_time(1_700_000_000)),
        AccessTokenVerification::Rejected(JwtRejection::Claims(ClaimsRejection::Expired { .. }))
    ));
}

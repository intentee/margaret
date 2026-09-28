use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client_tests::verifier_holding::verifier_holding;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn public_jwks_verifier_reports_claims_of_another_shape() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let token = secret
        .current()
        .sign_json(&json!("not the expected claims"), JwtType::AccessToken);

    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    assert!(matches!(
        verifier_holding(key_set).verify::<TestClaims>(&token, unix_time(1_700_000_000)),
        AccessTokenVerification::Rejected(JwtRejection::Claims(ClaimsRejection::Malformed { .. }))
    ));
}

use serde_json::json;

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client_tests::verifier_holding::verifier_holding;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn public_jwks_verifier_rejects_a_token_of_another_issuer() {
    let trust = fixture_trust();
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let token = secret.current().sign_json(
        &json!({
            "aud": trust.audience.as_str(),
            "client_id": "https://attacker.example",
            "exp": 1_700_000_060,
            "iat": 1_699_999_000,
            "iss": "https://attacker.example",
            "jti": "00000000-0000-0000-0000-000000000001",
            "sub": "00000000-0000-0000-0000-000000000002",
        }),
        JwtType::AccessToken,
    );
    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    assert!(matches!(
        verifier_holding(key_set).verify::<AccessTokenClaims>(&token, unix_time(1_700_000_000)),
        AccessTokenVerification::Rejected(JwtRejection::Claims(ClaimsRejection::IssuerMismatch { found, .. }))
            if found == "https://attacker.example"
    ));
}

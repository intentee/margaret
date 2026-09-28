use uuid::Uuid;

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client_tests::verifier_holding::verifier_holding;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::type_rejection::TypeRejection;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn public_jwks_verifier_rejects_a_refresh_token() {
    let trust = fixture_trust();
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let refresh_token = secret.current().sign_json(
        &RefreshTokenClaims {
            jti: Uuid::from_u128(1),
            sub: Uuid::from_u128(2),
        }
        .to_payload(&RegisteredClaims {
            aud: AudienceClaim::Single(trust.audience.as_str().to_string()),
            exp: NumericDate::new(1_700_000_060),
            iat: NumericDate::new(1_699_999_000),
            iss: trust.issuer.as_str().to_string(),
            nbf: None,
        }),
        JwtType::Jwt,
    );
    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    assert!(matches!(
        verifier_holding(key_set)
            .verify::<AccessTokenClaims>(&refresh_token, unix_time(1_700_000_000)),
        AccessTokenVerification::Rejected(JwtRejection::Type(TypeRejection::Mismatch {
            expected: JwtType::AccessToken,
            found: HeaderType::Supported(JwtType::Jwt),
        }))
    ));
}

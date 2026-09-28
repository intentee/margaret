use uuid::Uuid;

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_a_refresh_token_with_claims_of_another_shape() {
    let secret = fresh_p256_secret();
    let access_claims = AccessTokenClaims {
        sub: Uuid::from_u128(2),
    };
    let token = secret
        .current()
        .sign_json(&access_claims.to_payload(&RegisteredClaims {
            exp: NumericDate::new(10_000),
            iat: NumericDate::new(0),
            nbf: None,
        }));

    assert!(matches!(
        mint_access_token(&secret, &token, unix_time(1_000)),
        AccessTokenMinting::RejectedRefreshToken(JwtRejection::Claims(
            ClaimsRejection::Malformed { .. }
        ))
    ));
}

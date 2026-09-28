use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_a_refresh_token_with_claims_of_another_shape() {
    let secret = fresh_p256_secret();
    let token = secret.current().sign_json(
        &json!({ "exp": 10_000, "iat": 0, "sub": "00000000-0000-0000-0000-000000000002" }),
        JwtType::Jwt,
    );

    assert!(matches!(
        mint_access_token(&secret, &token, unix_time(1_000)),
        AccessTokenMinting::RejectedRefreshToken(JwtRejection::Claims(
            ClaimsRejection::Malformed { .. }
        ))
    ));
}

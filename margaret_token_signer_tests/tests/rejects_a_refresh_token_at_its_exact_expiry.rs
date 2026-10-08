use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_a_refresh_token_at_its_exact_expiry() {
    let issuance = fixture_issuance();
    let secret = fresh_secret(SigningCurve::P256);
    let refresh_token = sign_refresh_token(secret.current(), &issuance, &refresh_claims(), 1_000);

    assert!(matches!(
        mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000)),
        AccessTokenMinting::RejectedRefreshToken(JwtRejection::Claims(
            ClaimsRejection::Expired { .. }
        ))
    ));
}

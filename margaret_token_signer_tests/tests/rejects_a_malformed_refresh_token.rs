use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_a_malformed_refresh_token() {
    let issuance = fixture_issuance();
    assert!(matches!(
        mint_access_token(
            &fresh_secret(SigningCurve::P256),
            &issuance,
            "not-a-valid-jwt",
            unix_time(1_000)
        ),
        AccessTokenMinting::RejectedRefreshToken(JwtRejection::Jws(JwsRejection::NotCompactJws))
    ));
}

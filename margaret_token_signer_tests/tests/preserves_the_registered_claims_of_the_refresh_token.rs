use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn preserves_the_registered_claims_of_the_refresh_token() {
    let issuance = fixture_issuance();
    let secret = fresh_p256_secret();
    let original = RegisteredClaims {
        aud: AudienceClaim::Multiple(vec![issuance.audience.as_str().to_string()]),
        exp: NumericDate::new(10_000),
        iat: NumericDate::new(0),
        iss: issuance.issuer.as_str().to_string(),
        nbf: Some(NumericDate::new(500)),
    };
    let refresh_token = secret
        .current()
        .sign_json(&refresh_claims().to_payload(&original), JwtType::Jwt);

    let AccessTokenMinting::Minted(MintedTokens { refresh_token, .. }) =
        mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000))
    else {
        panic!("a valid refresh token mints new tokens");
    };
    let JwksSecretVerificationResult::SignedWithCurrent(reissued) = secret
        .verify_jwt::<RefreshTokenClaims>(
            &refresh_token,
            &JwtExpectation {
                audience: &issuance.audience,
                issuer: &issuance.issuer,
                token_type: TypeHeaderExpectation::Required(JwtType::Jwt),
            },
            NumericDate::new(1_000),
        )
    else {
        panic!("the reissued refresh token verifies");
    };

    assert_eq!(reissued.registered, original);
}

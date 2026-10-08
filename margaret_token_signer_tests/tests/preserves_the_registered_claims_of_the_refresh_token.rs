use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::refresh_token_profile::RefreshTokenProfile;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::minted_token_verification::minted_token_verification;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::refresh_token_identifier::REFRESH_TOKEN_IDENTIFIER;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn preserves_the_registered_claims_of_the_refresh_token() {
    let issuance = fixture_issuance();
    let secret = fresh_secret(SigningCurve::P256);
    let original = RegisteredClaims {
        aud: AudienceClaim::Multiple(vec![issuance.audience.to_string()]),
        exp: NumericDate::new(10_000),
        iat: Some(NumericDate::new(0)),
        iss: issuance.issuer.to_string(),
        jti: Some(REFRESH_TOKEN_IDENTIFIER.to_string()),
        nbf: Some(NumericDate::new(500)),
    };
    let refresh_token = secret
        .current()
        .sign_json(&refresh_claims().to_payload(&original), JwtType::Refresh);

    let AccessTokenMinting::Minted(MintedTokens { refresh_token, .. }) =
        mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000))
    else {
        panic!("a valid refresh token mints new tokens");
    };
    let JwtVerification::Verified(reissued) =
        minted_token_verification::<RefreshTokenClaims, RefreshTokenProfile>(
            &secret,
            &issuance,
            &refresh_token,
            NumericDate::new(1_000),
        )
    else {
        panic!("the minted token verifies");
    };

    assert_eq!(reissued.kid.as_ref(), Some(secret.current().kid()));

    assert_eq!(reissued.registered, original);
}

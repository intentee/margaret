use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn mints_from_a_current_key_refresh_token() {
    let issuance = fixture_issuance();
    let secret = fresh_p256_secret();
    let refresh = refresh_claims();
    let refresh_token = sign_refresh_token(secret.current(), &issuance, &refresh, 10_000);

    let AccessTokenMinting::Minted(MintedTokens { access_token, .. }) =
        mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000))
    else {
        panic!("a valid refresh token mints an access token");
    };
    let JwksSecretVerificationResult::SignedWithCurrent(access) = secret
        .verify_jwt::<AccessTokenClaims, AccessTokenProfile>(
            &access_token,
            &issuance.expectation(),
            NumericDate::new(1_000),
        )
    else {
        panic!("the minted access token verifies");
    };

    assert_eq!(access.claims.sub, refresh.sub);
    assert_eq!(
        access.registered,
        RegisteredClaims {
            aud: AudienceClaim::Single(issuance.audience.as_str().to_string()),
            exp: NumericDate::new(1_000 + i64::from(ACCESS_TOKEN_LIFETIME_SECS)),
            iat: NumericDate::new(1_000),
            iss: issuance.issuer.as_str().to_string(),
            nbf: None,
        }
    );
}

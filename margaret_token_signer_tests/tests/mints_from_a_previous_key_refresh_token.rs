use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::refresh_token_profile::RefreshTokenProfile;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn mints_from_a_previous_key_refresh_token() {
    let issuance = fixture_issuance();
    let secret = fresh_p256_secret()
        .rotate(&FixtureRsaSigningKeys::default())
        .expect("the fixture secret rotates");
    let PreviousKey::Retired(retired) = secret.previous() else {
        panic!("a rotated secret retires its previous key");
    };
    let refresh = refresh_claims();
    let refresh_token = sign_refresh_token(retired, &issuance, &refresh, 10_000);

    let AccessTokenMinting::Minted(MintedTokens {
        access_token,
        refresh_token,
    }) = mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000))
    else {
        panic!("a refresh token signed by the previous key mints an access token");
    };
    let JwksSecretVerificationResult::SignedWithCurrent(access) = secret
        .verify_jwt::<AccessTokenClaims, AccessTokenProfile>(
            &access_token,
            &JwtExpectation {
                audience: &issuance.audience,
                issuer: &issuance.issuer,
            },
            NumericDate::new(1_000),
        )
    else {
        panic!("the minted access token is signed with the current key");
    };
    let JwksSecretVerificationResult::SignedWithCurrent(migrated) = secret
        .verify_jwt::<RefreshTokenClaims, RefreshTokenProfile>(
            &refresh_token,
            &JwtExpectation {
                audience: &issuance.audience,
                issuer: &issuance.issuer,
            },
            NumericDate::new(1_000),
        )
    else {
        panic!("the migrated refresh token is signed with the current key");
    };

    assert_eq!(access.claims.sub, refresh.sub);
    assert_eq!(migrated.claims.jti, refresh.jti);
}

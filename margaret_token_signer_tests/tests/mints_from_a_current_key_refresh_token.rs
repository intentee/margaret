use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::minted_token_verification::minted_token_verification;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn mints_from_a_current_key_refresh_token() {
    let issuance = fixture_issuance();
    let secret = fresh_secret(SigningCurve::P256);
    let refresh = refresh_claims();
    let refresh_token = sign_refresh_token(secret.current(), &issuance, &refresh, 10_000);

    let AccessTokenMinting::Minted(MintedTokens { access_token, .. }) =
        mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000))
    else {
        panic!("a valid refresh token mints an access token");
    };
    let JwtVerification::Verified(access) =
        minted_token_verification::<AccessTokenClaims, AccessTokenProfile>(
            &secret,
            &issuance,
            &access_token,
            NumericDate::new(1_000),
        )
    else {
        panic!("the minted token verifies");
    };

    assert_eq!(access.kid.as_ref(), Some(secret.current().kid()));

    assert_eq!(access.claims.sub, refresh.sub);
    assert!(access.registered.jti.is_some());
    assert_eq!(
        RegisteredClaims {
            jti: None,
            ..access.registered
        },
        RegisteredClaims {
            aud: AudienceClaim::Single(issuance.audience.to_string()),
            exp: NumericDate::new(1_000 + i64::from(ACCESS_TOKEN_LIFETIME_SECS)),
            iat: Some(NumericDate::new(1_000)),
            iss: issuance.issuer.to_string(),
            jti: None,
            nbf: None,
        }
    );
}

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::refresh_token_profile::RefreshTokenProfile;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::minted_token_verification::minted_token_verification;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::refresh_token_identifier::REFRESH_TOKEN_IDENTIFIER;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn mints_from_a_retired_key_refresh_token() {
    let issuance = fixture_issuance();
    let fresh = fresh_secret(SigningCurve::P256);
    let secret = rolled_secret(&fresh);
    let refresh = refresh_claims();
    let refresh_token = sign_refresh_token(fresh.current(), &issuance, &refresh, 10_000);

    let AccessTokenMinting::Minted(MintedTokens {
        access_token,
        refresh_token,
    }) = mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000))
    else {
        panic!("a refresh token signed by a retired key mints an access token");
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

    let JwtVerification::Verified(migrated) =
        minted_token_verification::<RefreshTokenClaims, RefreshTokenProfile>(
            &secret,
            &issuance,
            &refresh_token,
            NumericDate::new(1_000),
        )
    else {
        panic!("the minted token verifies");
    };

    assert_eq!(access.kid.as_ref(), Some(secret.current().kid()));
    assert_eq!(migrated.kid.as_ref(), Some(secret.current().kid()));
    assert_eq!(access.claims.sub, refresh.sub);
    assert_eq!(
        migrated.registered.jti,
        Some(REFRESH_TOKEN_IDENTIFIER.to_string())
    );
}

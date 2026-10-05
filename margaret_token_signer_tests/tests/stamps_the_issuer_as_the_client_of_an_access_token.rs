use serde::Deserialize;

use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[derive(Deserialize)]
struct AccessTokenClient {
    client_id: String,
}

#[test]
fn stamps_the_issuer_as_the_client_of_an_access_token() {
    let issuance = fixture_issuance();
    let secret = fresh_p256_secret();
    let refresh_token = sign_refresh_token(secret.current(), &issuance, &refresh_claims(), 10_000);
    let AccessTokenMinting::Minted(MintedTokens { access_token, .. }) =
        mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000))
    else {
        panic!("a valid refresh token mints an access token");
    };
    let JwksSecretVerificationResult::SignedWithCurrent(access) = secret
        .verify_jwt::<AccessTokenClient, AccessTokenProfile>(
            &access_token,
            &issuance.expectation(),
            NumericDate::new(1_000),
        )
    else {
        panic!("the minted access token verifies");
    };

    assert_eq!(access.claims.client_id, issuance.issuer.as_str());
}

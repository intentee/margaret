use std::iter;
use std::time::Duration;

use margaret_identity_session::refresh_token_lifetime_secs::REFRESH_TOKEN_LIFETIME_SECS;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::roll_due_at::roll_due_at;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn reports_a_refresh_token_at_the_end_of_its_lifetime_as_expired() {
    let issuance = fixture_issuance();
    let fresh = fresh_secret(SigningCurve::P256);
    let signed_just_before_the_first_roll =
        NumericDate::new(roll_due_at(&fresh).seconds_since_epoch() - 1);
    let expires_at = signed_just_before_the_first_roll
        .after(Duration::from_secs(u64::from(REFRESH_TOKEN_LIFETIME_SECS)))
        .seconds_since_epoch();
    let refresh_token =
        sign_refresh_token(fresh.current(), &issuance, &refresh_claims(), expires_at);
    let generations: Vec<JwksSecret> =
        iter::successors(Some(fresh), |secret| Some(rolled_secret(secret)))
            .take_while(|secret| secret.rolled_at().seconds_since_epoch() < expires_at)
            .collect();
    let last = generations
        .last()
        .expect("the fresh secret starts the generations");

    assert!(matches!(
        mint_access_token(last, &issuance, &refresh_token, unix_time(expires_at - 1)),
        AccessTokenMinting::Minted(_)
    ));
    assert!(matches!(
        mint_access_token(last, &issuance, &refresh_token, unix_time(expires_at)),
        AccessTokenMinting::RejectedRefreshToken(JwtRejection::Claims(
            ClaimsRejection::Expired { .. }
        ))
    ));
}

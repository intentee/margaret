use serde::de::DeserializeOwned;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_issuance::token_issuance::TokenIssuance;

#[must_use]
pub fn minted_token_verification<TClaims: DeserializeOwned, TProfile: JwtProfile>(
    secret: &JwksSecret,
    issuance: &TokenIssuance,
    token: &str,
    now: NumericDate,
) -> JwtVerification<TClaims, TProfile> {
    verify_serialized_jwt(secret.token_key_set(), token, &issuance.expectation(), now)
}

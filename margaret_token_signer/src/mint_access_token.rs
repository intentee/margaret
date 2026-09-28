use chrono::DateTime;
use chrono::Utc;

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::access_token_minting::AccessTokenMinting;
use crate::minted_tokens::MintedTokens;

#[must_use]
pub fn mint_access_token(
    secret: &JwksSecret,
    refresh_token: &str,
    now: DateTime<Utc>,
) -> AccessTokenMinting {
    let access_registered = RegisteredClaims::issued_at(now, ACCESS_TOKEN_LIFETIME_SECS);
    let VerifiedJwt {
        claims: refresh_claims,
        registered: refresh_registered,
        ..
    } = match secret.verify_jwt::<RefreshTokenClaims>(refresh_token, access_registered.iat) {
        JwksSecretVerificationResult::Rejected(rejection) => {
            return AccessTokenMinting::RejectedRefreshToken(rejection);
        }
        JwksSecretVerificationResult::SignedWithNextKey => {
            return AccessTokenMinting::RefreshTokenSignedWithNextKey;
        }
        JwksSecretVerificationResult::SignedWithCurrent(verified)
        | JwksSecretVerificationResult::SignedWithPrevious(verified) => verified,
    };
    let signer = secret.current();
    let access_claims = AccessTokenClaims {
        sub: refresh_claims.sub,
    };

    AccessTokenMinting::Minted(MintedTokens {
        access_token: signer.sign_json(&access_claims.to_payload(&access_registered)),
        refresh_token: signer.sign_json(&refresh_claims.to_payload(&refresh_registered)),
    })
}

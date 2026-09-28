use chrono::DateTime;
use chrono::Utc;

use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;

use crate::access_token_minting::AccessTokenMinting;
use crate::minted_tokens::MintedTokens;

#[must_use]
pub fn mint_access_token(
    secret: &JwksSecret,
    refresh_token: &str,
    now: DateTime<Utc>,
) -> AccessTokenMinting {
    let refresh_claims = match secret.verify_any::<RefreshTokenClaims>(refresh_token) {
        JwksSecretVerificationResult::MalformedClaims(source) => {
            return AccessTokenMinting::MalformedRefreshTokenClaims(source);
        }
        JwksSecretVerificationResult::Rejected(rejection) => {
            return AccessTokenMinting::RejectedRefreshToken(rejection);
        }
        JwksSecretVerificationResult::SignedWithNextKey => {
            return AccessTokenMinting::RefreshTokenSignedWithNextKey;
        }
        JwksSecretVerificationResult::SignedWithCurrent(claims)
        | JwksSecretVerificationResult::SignedWithPrevious(claims) => claims,
    };

    if refresh_claims.is_expired_at(now) {
        return AccessTokenMinting::ExpiredRefreshToken;
    }

    let signer = secret.current();

    AccessTokenMinting::Minted(MintedTokens {
        access_token: signer.sign_json(&refresh_claims.mint_access_token_claims(now).to_json()),
        refresh_token: signer.sign_json(&refresh_claims.to_json()),
    })
}

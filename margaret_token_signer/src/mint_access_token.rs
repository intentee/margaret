use chrono::DateTime;
use chrono::Utc;
use futures_util::future::try_join;
use serde::Serialize;

use margaret_identity_session::is_expired::IsExpired;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_any_token::VerifiesAnyToken;

use crate::minted_tokens::MintedTokens;
use crate::token_signer_error::TokenSignerError;

async fn sign_with_current<TClaims: Send + Serialize + Sync>(
    secret: &JwksSecret,
    claims: &TClaims,
) -> Result<String, TokenSignerError> {
    secret
        .current
        .signing
        .sign(claims)
        .await
        .map_err(|source| TokenSignerError::Signing { source })
}

pub async fn mint_access_token(
    secret: &JwksSecret,
    refresh_token: &str,
    now: DateTime<Utc>,
) -> Result<MintedTokens, TokenSignerError> {
    let verification = secret
        .verify_any::<RefreshTokenClaims>(refresh_token)
        .map_err(|source| TokenSignerError::UnverifiableRefreshToken { source })?;

    let refresh_claims = match verification {
        JwksSecretVerificationResult::Invalid => {
            return Err(TokenSignerError::InvalidRefreshToken);
        }
        JwksSecretVerificationResult::SignedWithCurrent(claims)
        | JwksSecretVerificationResult::SignedWithPrevious(claims) => claims,
    };

    if refresh_claims.is_expired(now) {
        return Err(TokenSignerError::ExpiredRefreshToken);
    }

    let access_claims = refresh_claims.mint_access_token_claims(now);
    let (access_token, refresh_token) = try_join(
        sign_with_current(secret, &access_claims),
        sign_with_current(secret, &refresh_claims),
    )
    .await?;

    Ok(MintedTokens {
        access_token,
        refresh_token,
    })
}

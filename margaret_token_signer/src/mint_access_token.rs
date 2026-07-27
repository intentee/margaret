use chrono::DateTime;
use chrono::Utc;
use futures_util::future::try_join;
use serde::Serialize;

use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_any_token::VerifiesAnyToken;

use crate::mint_access_token_outcome::MintAccessTokenOutcome;
use crate::minted_tokens::MintedTokens;
use crate::refresh_token_rejection::RefreshTokenRejection;
use crate::token_signer_error::TokenSignerError;

async fn sign_with_current<TClaims: Send + Serialize + Sync>(
    secret: &JwksSecret,
    claims: &TClaims,
) -> Result<String, JwksKeyError> {
    secret.current.signing.sign(claims).await
}

pub async fn mint_access_token(
    secret: &JwksSecret,
    refresh_token: &str,
    now: DateTime<Utc>,
) -> Result<MintAccessTokenOutcome, TokenSignerError> {
    let verification = match secret.verify_any::<RefreshTokenClaims>(refresh_token) {
        Ok(verification) => verification,
        Err(
            JwksKeyError::AlgorithmMismatch { .. }
            | JwksKeyError::ClaimsBase64 { .. }
            | JwksKeyError::ClaimsJson { .. }
            | JwksKeyError::HeaderBase64 { .. }
            | JwksKeyError::HeaderJson { .. }
            | JwksKeyError::MalformedCompactJws
            | JwksKeyError::NonCanonicalSignature
            | JwksKeyError::SignatureBase64 { .. }
            | JwksKeyError::SignatureMalformed { .. }
            | JwksKeyError::SignatureMismatch
            | JwksKeyError::UnknownKeyId { .. },
        ) => {
            return Ok(MintAccessTokenOutcome::Rejected(
                RefreshTokenRejection::Invalid,
            ));
        }
        Err(source) => return Err(TokenSignerError::Verification { source }),
    };

    let refresh_claims = match verification {
        JwksSecretVerificationResult::Invalid => {
            return Ok(MintAccessTokenOutcome::Rejected(
                RefreshTokenRejection::Invalid,
            ));
        }
        JwksSecretVerificationResult::SignedWithCurrent(claims)
        | JwksSecretVerificationResult::SignedWithPrevious(claims) => claims,
    };

    if refresh_claims.is_expired_at(now) {
        return Ok(MintAccessTokenOutcome::Rejected(
            RefreshTokenRejection::Expired,
        ));
    }

    let access_claims = refresh_claims.mint_access_token_claims(now);
    let (access_token, refresh_token) = try_join(
        sign_with_current(secret, &access_claims),
        sign_with_current(secret, &refresh_claims),
    )
    .await
    .map_err(|source| TokenSignerError::Signing { source })?;

    Ok(MintAccessTokenOutcome::Minted(MintedTokens {
        access_token,
        refresh_token,
    }))
}

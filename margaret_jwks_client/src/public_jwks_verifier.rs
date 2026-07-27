use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_identity_session::is_expired::IsExpired;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken as _;

use crate::jwks_client_error::JwksClientError;
use crate::public_jwks_holder::PublicJwksHolder;
use crate::public_token_verification::PublicTokenVerification;
use crate::token_rejection::TokenRejection;

fn verification_error<TClaims>(
    error: JwksKeyError,
) -> Result<PublicTokenVerification<TClaims>, JwksClientError> {
    match error {
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
        | JwksKeyError::UnknownKeyId { .. } => {
            Ok(PublicTokenVerification::Rejected(TokenRejection::Invalid))
        }
        JwksKeyError::CoordinateBase64 { .. }
        | JwksKeyError::DuplicateKeyId { .. }
        | JwksKeyError::MalformedVerifyingKey { .. }
        | JwksKeyError::MissingPublicKeyCoordinate { .. }
        | JwksKeyError::PemEncoding { .. }
        | JwksKeyError::SigningKeyRejected { .. } => Err(JwksClientError::TokenVerification(error)),
    }
}

pub struct PublicJwksVerifier {
    public_jwks_holder: PublicJwksHolder,
}

impl PublicJwksVerifier {
    #[must_use]
    pub fn new(public_jwks_holder: PublicJwksHolder) -> Self {
        Self { public_jwks_holder }
    }

    pub fn verify<TClaims: DeserializeOwned + IsExpired>(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<PublicTokenVerification<TClaims>, JwksClientError> {
        let public_jwks = self
            .public_jwks_holder
            .get()
            .ok_or(JwksClientError::NotReady)?;

        let claims: TClaims = match public_jwks.verify(token) {
            Ok(TokenVerification::Verified(claims)) => claims,
            Ok(TokenVerification::SignatureMismatch) => {
                return Ok(PublicTokenVerification::Rejected(TokenRejection::Invalid));
            }
            Err(error) => return verification_error(error),
        };

        if claims
            .is_expired(now)
            .map_err(|source| JwksClientError::TokenExpiry { source })?
        {
            return Ok(PublicTokenVerification::Rejected(TokenRejection::Expired));
        }

        Ok(PublicTokenVerification::Verified(claims))
    }
}

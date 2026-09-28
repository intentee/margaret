use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_identity_session::is_expired::IsExpired;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::verified_jws::VerifiedJws;

use crate::access_token_verification::AccessTokenVerification;
use crate::jwks_client_error::JwksClientError;
use crate::verification_key_set_holder::VerificationKeySetHolder;

pub struct PublicJwksVerifier {
    verification_key_set_holder: VerificationKeySetHolder,
}

impl PublicJwksVerifier {
    #[must_use]
    pub fn new(verification_key_set_holder: VerificationKeySetHolder) -> Self {
        Self {
            verification_key_set_holder,
        }
    }

    /// # Errors
    ///
    /// Returns `JwksClientError::TokenExpiry` when the claims cannot tell whether they expired.
    pub fn verify<TClaims: DeserializeOwned + IsExpired>(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<AccessTokenVerification<TClaims>, JwksClientError> {
        let Some(key_set) = self.verification_key_set_holder.get() else {
            return Ok(AccessTokenVerification::NotReady);
        };
        let VerifiedJws { payload, .. } = match key_set.verify(token) {
            JwsVerification::Rejected(rejection) => {
                return Ok(AccessTokenVerification::Rejected(rejection));
            }
            JwsVerification::Verified(verified) => verified,
        };
        let claims: TClaims = match serde_json::from_slice(&payload) {
            Ok(claims) => claims,
            Err(source) => return Ok(AccessTokenVerification::MalformedClaims(source)),
        };

        if claims
            .is_expired(now)
            .map_err(|source| JwksClientError::TokenExpiry { source })?
        {
            return Ok(AccessTokenVerification::Expired);
        }

        Ok(AccessTokenVerification::Verified(claims))
    }
}

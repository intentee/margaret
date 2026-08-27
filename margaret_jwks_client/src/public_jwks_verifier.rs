use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_identity_session::accepts_claims::AcceptsClaims;
use margaret_identity_session::claims_acceptance::ClaimsAcceptance;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken as _;

use crate::access_token_verification::AccessTokenVerification;
use crate::jwks_client_error::JwksClientError;
use crate::public_jwks_holder::PublicJwksHolder;

pub struct PublicJwksVerifier {
    public_jwks_holder: PublicJwksHolder,
}

impl PublicJwksVerifier {
    #[must_use]
    pub fn new(public_jwks_holder: PublicJwksHolder) -> Self {
        Self { public_jwks_holder }
    }

    /// # Errors
    ///
    /// Returns `JwksClientError::TokenVerification` or `JwksClientError::ClaimsAcceptance`.
    pub fn verify<TClaims: AcceptsClaims + DeserializeOwned>(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<AccessTokenVerification<TClaims>, JwksClientError> {
        let Some(public_jwks) = self.public_jwks_holder.get() else {
            return Ok(AccessTokenVerification::NotReady);
        };

        let claims: TClaims = match public_jwks
            .verify(token)
            .map_err(JwksClientError::TokenVerification)?
        {
            TokenVerification::Malformed(malformation) => {
                return Ok(AccessTokenVerification::Malformed(malformation));
            }
            TokenVerification::SignatureMismatch => {
                return Ok(AccessTokenVerification::SignatureMismatch);
            }
            TokenVerification::Verified(claims) => claims,
        };

        match claims
            .accepts(now)
            .map_err(|source| JwksClientError::ClaimsAcceptance { source })?
        {
            ClaimsAcceptance::Accepted => Ok(AccessTokenVerification::Verified(claims)),
            ClaimsAcceptance::Rejected(rejection) => {
                Ok(AccessTokenVerification::Rejected(rejection))
            }
        }
    }
}

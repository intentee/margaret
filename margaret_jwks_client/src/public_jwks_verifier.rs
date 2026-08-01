use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_identity_session::is_expired::IsExpired;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken as _;
use margaret_jwt_claims::expected_claims::ExpectedClaims;
use margaret_jwt_claims::has_audience::HasAudience;
use margaret_jwt_claims::has_issuer::HasIssuer;

use crate::access_token_verification::AccessTokenVerification;
use crate::jwks_client_error::JwksClientError;
use crate::public_jwks_holder::PublicJwksHolder;

pub struct PublicJwksVerifier {
    expected_claims: ExpectedClaims,
    public_jwks_holder: PublicJwksHolder,
}

impl PublicJwksVerifier {
    #[must_use]
    pub fn new(expected_claims: ExpectedClaims, public_jwks_holder: PublicJwksHolder) -> Self {
        Self {
            expected_claims,
            public_jwks_holder,
        }
    }

    /// # Errors
    ///
    /// Returns `JwksClientError::TokenVerification` or `JwksClientError::TokenExpiry`.
    pub fn verify<TClaims: DeserializeOwned + HasAudience + HasIssuer + IsExpired>(
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

        if claims.issuer() != self.expected_claims.issuer {
            return Ok(AccessTokenVerification::IssuerMismatch);
        }

        if !claims.audience().contains(&self.expected_claims.audience) {
            return Ok(AccessTokenVerification::AudienceMismatch);
        }

        if claims
            .is_expired(now)
            .map_err(|source| JwksClientError::TokenExpiry { source })?
        {
            return Ok(AccessTokenVerification::Expired);
        }

        Ok(AccessTokenVerification::Verified(claims))
    }
}

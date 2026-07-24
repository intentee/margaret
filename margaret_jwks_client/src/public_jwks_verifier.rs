use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_identity_session::is_expired::IsExpired;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken as _;

use crate::jwks_client_error::JwksClientError;
use crate::public_jwks_holder::PublicJwksHolder;

#[derive(Default)]
pub struct PublicJwksVerifier {
    public_jwks_holder: PublicJwksHolder,
}

impl PublicJwksVerifier {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.public_jwks_holder.is_ready()
    }

    #[must_use]
    pub fn public_jwks_holder(&self) -> PublicJwksHolder {
        self.public_jwks_holder.clone()
    }

    pub fn verify<TClaims: DeserializeOwned + IsExpired>(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<TClaims, JwksClientError> {
        let public_jwks = self
            .public_jwks_holder
            .get()
            .ok_or(JwksClientError::NotReady)?;

        let claims: TClaims = public_jwks
            .verify(token)
            .and_then(TokenVerification::must)
            .map_err(JwksClientError::TokenVerification)?;

        if claims.is_expired(now) {
            return Err(JwksClientError::TokenExpired);
        }

        Ok(claims)
    }
}

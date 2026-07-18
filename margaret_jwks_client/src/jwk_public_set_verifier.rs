use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_identity_session::is_expired::IsExpired;
use margaret_jwks_key_gen::token_verification::TokenVerification;
use margaret_jwks_key_gen::verifies_token::VerifiesToken as _;

use crate::jwk_public_set_holder::JwkPublicSetHolder;
use crate::jwks_client_error::JwksClientError;

pub struct JwkPublicSetVerifier {
    jwk_public_set_holder: JwkPublicSetHolder,
}

impl JwkPublicSetVerifier {
    #[must_use]
    pub fn new(jwk_public_set_holder: JwkPublicSetHolder) -> Self {
        Self {
            jwk_public_set_holder,
        }
    }

    pub fn verify<TClaims: DeserializeOwned + IsExpired>(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<TClaims, JwksClientError> {
        let jwk_public_set = self
            .jwk_public_set_holder
            .get()
            .ok_or(JwksClientError::NotReady)?;

        let claims: TClaims = jwk_public_set
            .verify(token)
            .and_then(TokenVerification::must)
            .map_err(JwksClientError::TokenVerification)?;

        if claims.is_expired(now) {
            return Err(JwksClientError::TokenExpired);
        }

        Ok(claims)
    }
}

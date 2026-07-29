use serde::de::DeserializeOwned;

use crate::jwks_key_error::JwksKeyError;
use crate::token_verification::TokenVerification;

pub trait VerifiesToken {
    /// # Errors
    ///
    /// Returns `JwksKeyError` propagated from the work it performs.
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError>;
}

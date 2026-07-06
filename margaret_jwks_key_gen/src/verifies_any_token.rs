use serde::de::DeserializeOwned;

use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret_verification_result::JwksSecretVerificationResult;

pub trait VerifiesAnyToken {
    fn verify_any<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<JwksSecretVerificationResult<TClaims>, JwksKeyError>;
}

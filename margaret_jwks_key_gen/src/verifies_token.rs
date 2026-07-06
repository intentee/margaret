use serde::de::DeserializeOwned;

use crate::jwks_key_error::JwksKeyError;
use crate::token_verification::TokenVerification;

pub trait VerifiesToken {
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError>;
}

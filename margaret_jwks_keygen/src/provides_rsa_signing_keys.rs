use crate::jwks_key_error::JwksKeyError;
use crate::rsa_signing_key::RsaSigningKey;

pub trait ProvidesRsaSigningKeys: Send + Sync {
    /// # Errors
    ///
    /// Returns `JwksKeyError` when a signing key cannot be provided.
    fn rsa_signing_key(&self) -> Result<RsaSigningKey, JwksKeyError>;
}

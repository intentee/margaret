use crate::jwks_key_error::JwksKeyError;
use crate::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use crate::rsa_signing_key::RsaSigningKey;

pub struct GeneratedRsaSigningKeys;

impl ProvidesRsaSigningKeys for GeneratedRsaSigningKeys {
    fn rsa_signing_key(&self) -> Result<RsaSigningKey, JwksKeyError> {
        RsaSigningKey::generate()
    }
}

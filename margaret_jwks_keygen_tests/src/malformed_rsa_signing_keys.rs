use zeroize::Zeroizing;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_keygen::rsa_signing_key::RsaSigningKey;

const NOT_A_PKCS8_DOCUMENT: &[u8] = b"not a pkcs8 document";

pub struct MalformedRsaSigningKeys;

impl ProvidesRsaSigningKeys for MalformedRsaSigningKeys {
    fn rsa_signing_key(&self) -> Result<RsaSigningKey, JwksKeyError> {
        RsaSigningKey::from_pkcs8(Zeroizing::new(NOT_A_PKCS8_DOCUMENT.to_vec()))
    }
}

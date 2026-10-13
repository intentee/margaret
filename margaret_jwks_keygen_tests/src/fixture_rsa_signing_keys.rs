use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use zeroize::Zeroizing;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_keygen::rsa_signing_key::RsaSigningKey;

const FIXTURE_KEYS: [&[u8]; 4] = [
    include_bytes!("../fixtures/rsa_signing_key_1.der"),
    include_bytes!("../fixtures/rsa_signing_key_2.der"),
    include_bytes!("../fixtures/rsa_signing_key_3.der"),
    include_bytes!("../fixtures/rsa_signing_key_4.der"),
];

#[derive(Default)]
pub struct FixtureRsaSigningKeys {
    cursor: AtomicUsize,
}

impl ProvidesRsaSigningKeys for FixtureRsaSigningKeys {
    fn rsa_signing_key(&self) -> Result<RsaSigningKey, JwksKeyError> {
        let index = self.cursor.fetch_add(1, Ordering::Relaxed) % FIXTURE_KEYS.len();

        RsaSigningKey::from_pkcs8(Zeroizing::new(FIXTURE_KEYS[index].to_vec()))
    }
}

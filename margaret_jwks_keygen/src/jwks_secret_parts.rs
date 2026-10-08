use margaret_registered_claims::numeric_date::NumericDate;

use crate::jwk_pair::JwkPair;
use crate::key_retention::KeyRetention;
use crate::retired_key::RetiredKey;
use crate::rsa_key_ring::RsaKeyRing;
use crate::signing_keys_generation::SigningKeysGeneration;

pub(crate) struct JwksSecretParts {
    pub(crate) current: JwkPair,
    pub(crate) generation: SigningKeysGeneration,
    pub(crate) next: JwkPair,
    pub(crate) retention: KeyRetention,
    pub(crate) retired: Vec<RetiredKey>,
    pub(crate) rolled_at: NumericDate,
    pub(crate) rsa: RsaKeyRing,
}

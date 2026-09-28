use margaret_jws_verification::key_id::KeyId;

use crate::jwk_pair::JwkPair;

#[derive(Clone)]
pub enum PreviousKey {
    Absent,
    Retired(Box<JwkPair>),
}

impl PreviousKey {
    #[must_use]
    pub fn is_retired_key(&self, kid: &KeyId) -> bool {
        matches!(self, Self::Retired(pair) if pair.kid() == kid)
    }
}

use uuid::Uuid;

use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;

use crate::jwks_key_error::JwksKeyError;
use crate::previous_key::PreviousKey;
use crate::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use crate::rsa_jwk_pair::RsaJwkPair;

fn provided_pair(keys: &dyn ProvidesRsaSigningKeys) -> Result<RsaJwkPair, JwksKeyError> {
    keys.rsa_signing_key()
        .map(|signing_key| RsaJwkPair::new(KeyId::new(Uuid::new_v4().to_string()), signing_key))
}

#[derive(Clone)]
pub struct RsaKeyRing {
    current: RsaJwkPair,
    next: RsaJwkPair,
    previous: PreviousKey<RsaJwkPair>,
}

impl RsaKeyRing {
    /// # Errors
    ///
    /// Returns `JwksKeyError` when the provider cannot provide a signing key.
    pub fn fresh(keys: &dyn ProvidesRsaSigningKeys) -> Result<Self, JwksKeyError> {
        provided_pair(keys).and_then(|current| {
            provided_pair(keys).map(|next| Self::new(current, next, PreviousKey::Absent))
        })
    }

    #[must_use]
    pub fn new(current: RsaJwkPair, next: RsaJwkPair, previous: PreviousKey<RsaJwkPair>) -> Self {
        Self {
            current,
            next,
            previous,
        }
    }

    #[must_use]
    pub fn current(&self) -> &RsaJwkPair {
        &self.current
    }

    #[must_use]
    pub fn next(&self) -> &RsaJwkPair {
        &self.next
    }

    #[must_use]
    pub fn previous(&self) -> &PreviousKey<RsaJwkPair> {
        &self.previous
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError` when the provider cannot provide the next signing key.
    pub fn rotate(&self, keys: &dyn ProvidesRsaSigningKeys) -> Result<Self, JwksKeyError> {
        provided_pair(keys).map(|next| {
            Self::new(
                self.next.clone(),
                next,
                PreviousKey::Retired(Box::new(self.current.clone())),
            )
        })
    }

    pub(crate) fn published(&self) -> Vec<Jwk> {
        let mut published = vec![self.current.public_jwk().clone()];

        if let PreviousKey::Retired(retired) = &self.previous {
            published.push(retired.public_jwk().clone());
        }

        published.push(self.next.public_jwk().clone());

        published
    }
}

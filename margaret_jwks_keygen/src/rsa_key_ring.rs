use std::iter;

use uuid::Uuid;

use margaret_jws_verification::key_id::KeyId;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::jwks_key_error::JwksKeyError;
use crate::key_retention::KeyRetention;
use crate::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use crate::retired_key::RetiredKey;
use crate::rsa_jwk_pair::RsaJwkPair;

fn provided_pair(keys: &dyn ProvidesRsaSigningKeys) -> Result<RsaJwkPair, JwksKeyError> {
    keys.rsa_signing_key().and_then(|signing_key| {
        RsaJwkPair::new(KeyId::new(Uuid::new_v4().to_string()), signing_key)
    })
}

#[derive(Clone)]
pub struct RsaKeyRing {
    current: RsaJwkPair,
    next: RsaJwkPair,
    retired: Vec<RetiredKey>,
}

impl RsaKeyRing {
    /// # Errors
    ///
    /// Returns `JwksKeyError` when the provider cannot provide a signing key.
    pub fn fresh(keys: &dyn ProvidesRsaSigningKeys) -> Result<Self, JwksKeyError> {
        provided_pair(keys).and_then(|current| {
            provided_pair(keys).map(|next| Self::new(current, next, Vec::new()))
        })
    }

    pub(crate) fn new(current: RsaJwkPair, next: RsaJwkPair, retired: Vec<RetiredKey>) -> Self {
        Self {
            current,
            next,
            retired,
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
    pub fn retired(&self) -> &[RetiredKey] {
        &self.retired
    }

    pub(crate) fn rolled(
        &self,
        keys: &dyn ProvidesRsaSigningKeys,
        retention: KeyRetention,
        now: NumericDate,
    ) -> Result<Self, JwksKeyError> {
        provided_pair(keys).map(|next| Self {
            current: self.next.clone(),
            next,
            retired: iter::once(RetiredKey::new(
                self.current.public_jwk().clone(),
                self.current.verification_key().clone(),
                now,
            ))
            .chain(
                self.retired
                    .iter()
                    .filter(|retired| retired.outlives(retention.token, now))
                    .cloned(),
            )
            .collect(),
        })
    }
}

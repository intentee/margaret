use std::iter;

use uuid::Uuid;

use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::ec_signing_key::EcSigningKey;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret_parts::JwksSecretParts;
use crate::key_entry::KeyEntry;
use crate::key_retention::KeyRetention;
use crate::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use crate::public_jwks::PublicJwks;
use crate::retired_key::RetiredKey;
use crate::rsa_jwk_pair::RsaJwkPair;
use crate::rsa_key_ring::RsaKeyRing;
use crate::signing_curve::SigningCurve;
use crate::signing_keys_generation::SigningKeysGeneration;

fn assembled(entries: &[KeyEntry]) -> Result<VerificationKeySet, JwksKeyError> {
    match VerificationKeySet::assemble(
        entries
            .iter()
            .map(|entry| entry.verification_key.clone())
            .collect(),
    ) {
        KeySetAssembly::Assembled(key_set) => Ok(key_set),
        KeySetAssembly::DuplicateKeyId(duplicate) => {
            Err(JwksKeyError::DuplicateKeyId { duplicate })
        }
    }
}

fn pair_entry(pair: &JwkPair) -> KeyEntry<'_> {
    KeyEntry {
        public_jwk: pair.public_jwk(),
        verification_key: pair.verification_key(),
    }
}

fn rsa_pair_entry(pair: &RsaJwkPair) -> KeyEntry<'_> {
    KeyEntry {
        public_jwk: pair.public_jwk(),
        verification_key: pair.verification_key(),
    }
}

fn retired_entry(retired: &RetiredKey) -> KeyEntry<'_> {
    KeyEntry {
        public_jwk: retired.public_jwk(),
        verification_key: retired.verification_key(),
    }
}

fn random_pair(curve: SigningCurve) -> Result<JwkPair, JwksKeyError> {
    EcSigningKey::generate(curve)
        .and_then(|signing_key| JwkPair::new(KeyId::new(Uuid::new_v4().to_string()), signing_key))
}

#[derive(Clone)]
pub struct JwksSecret {
    current: JwkPair,
    generation: SigningKeysGeneration,
    next: JwkPair,
    public_jwks: PublicJwks,
    published_key_set: VerificationKeySet,
    refresh_key_set: VerificationKeySet,
    retention: KeyRetention,
    retired: Vec<RetiredKey>,
    rolled_at: NumericDate,
    rsa: RsaKeyRing,
    token_key_set: VerificationKeySet,
}

impl JwksSecret {
    pub(crate) fn assembled(
        JwksSecretParts {
            current,
            generation,
            next,
            retention,
            retired,
            rolled_at,
            rsa,
        }: JwksSecretParts,
    ) -> Result<Self, JwksKeyError> {
        let token_entries: Vec<KeyEntry> = [pair_entry(&current), pair_entry(&next)]
            .into_iter()
            .chain(
                retired
                    .iter()
                    .filter(|retired| retired.outlives(retention.token, rolled_at))
                    .map(retired_entry),
            )
            .collect();
        let refresh_entries: Vec<KeyEntry> = [pair_entry(&current), pair_entry(&next)]
            .into_iter()
            .chain(retired.iter().map(retired_entry))
            .collect();
        let published_entries: Vec<KeyEntry> = token_entries
            .iter()
            .copied()
            .chain([rsa_pair_entry(rsa.current()), rsa_pair_entry(rsa.next())])
            .chain(rsa.retired().iter().map(retired_entry))
            .collect();
        let token_key_set = assembled(&token_entries)?;
        let refresh_key_set = assembled(&refresh_entries)?;
        let published_key_set = assembled(&published_entries)?;
        let public_jwks = PublicJwks::new(
            published_entries
                .iter()
                .map(|entry| entry.public_jwk.clone())
                .collect(),
        );

        Ok(Self {
            current,
            generation,
            next,
            public_jwks,
            published_key_set,
            refresh_key_set,
            retention,
            retired,
            rolled_at,
            rsa,
            token_key_set,
        })
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError` when a key cannot be generated or the keys do not form a key set.
    pub fn fresh(
        curve: SigningCurve,
        rsa_keys: &dyn ProvidesRsaSigningKeys,
        retention: KeyRetention,
        now: NumericDate,
    ) -> Result<Self, JwksKeyError> {
        random_pair(curve).and_then(|current| {
            random_pair(curve).and_then(|next| {
                RsaKeyRing::fresh(rsa_keys).and_then(|rsa| {
                    Self::assembled(JwksSecretParts {
                        current,
                        generation: SigningKeysGeneration::FIRST,
                        next,
                        retention,
                        retired: Vec::new(),
                        rolled_at: now,
                        rsa,
                    })
                })
            })
        })
    }

    #[must_use]
    pub fn current(&self) -> &JwkPair {
        &self.current
    }

    #[must_use]
    pub fn generation(&self) -> SigningKeysGeneration {
        self.generation
    }

    #[must_use]
    pub fn next(&self) -> &JwkPair {
        &self.next
    }

    #[must_use]
    pub fn public_jwks(&self) -> &PublicJwks {
        &self.public_jwks
    }

    #[must_use]
    pub fn published_key_set(&self) -> &VerificationKeySet {
        &self.published_key_set
    }

    #[must_use]
    pub fn refresh_key_set(&self) -> &VerificationKeySet {
        &self.refresh_key_set
    }

    #[must_use]
    pub fn retired(&self) -> &[RetiredKey] {
        &self.retired
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError` when no generation follows this one, the next keys cannot be
    /// generated, or the keys do not form a key set.
    pub fn rolled(
        &self,
        rsa_keys: &dyn ProvidesRsaSigningKeys,
        now: NumericDate,
    ) -> Result<Self, JwksKeyError> {
        let generation = self.generation.successor()?;

        random_pair(self.current.signing_key().curve()).and_then(|next| {
            self.rsa
                .rolled(rsa_keys, self.retention, now)
                .and_then(|rsa| {
                    Self::assembled(JwksSecretParts {
                        current: self.next.clone(),
                        generation,
                        next,
                        retention: self.retention,
                        retired: iter::once(RetiredKey::new(
                            self.current.public_jwk().clone(),
                            self.current.verification_key().clone(),
                            now,
                        ))
                        .chain(
                            self.retired
                                .iter()
                                .filter(|retired| retired.outlives(self.retention.refresh, now))
                                .cloned(),
                        )
                        .collect(),
                        rolled_at: now,
                        rsa,
                    })
                })
        })
    }

    #[must_use]
    pub fn rolled_at(&self) -> NumericDate {
        self.rolled_at
    }

    #[must_use]
    pub fn rsa(&self) -> &RsaKeyRing {
        &self.rsa
    }

    #[must_use]
    pub fn token_key_set(&self) -> &VerificationKeySet {
        &self.token_key_set
    }
}

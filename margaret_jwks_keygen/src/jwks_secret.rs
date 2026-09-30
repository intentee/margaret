use serde::de::DeserializeOwned;
use uuid::Uuid;

use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::ec_signing_key::EcSigningKey;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret_verification_result::JwksSecretVerificationResult;
use crate::previous_key::PreviousKey;
use crate::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use crate::public_jwks::PublicJwks;
use crate::rsa_key_ring::RsaKeyRing;
use crate::signing_curve::SigningCurve;

fn random_pair(curve: SigningCurve) -> Result<JwkPair, JwksKeyError> {
    EcSigningKey::generate(curve)
        .and_then(|signing_key| JwkPair::new(KeyId::new(Uuid::new_v4().to_string()), signing_key))
}

#[derive(Clone)]
pub struct JwksSecret {
    current: JwkPair,
    key_set: VerificationKeySet,
    next: JwkPair,
    previous: PreviousKey<JwkPair>,
    public_jwks: PublicJwks,
    rsa: RsaKeyRing,
}

impl JwksSecret {
    /// # Errors
    ///
    /// Returns `JwksKeyError` when a key cannot be generated or the keys do not form a key set.
    pub fn fresh(
        curve: SigningCurve,
        rsa_keys: &dyn ProvidesRsaSigningKeys,
    ) -> Result<Self, JwksKeyError> {
        random_pair(curve).and_then(|current| {
            random_pair(curve).and_then(|next| {
                RsaKeyRing::fresh(rsa_keys)
                    .and_then(|rsa| Self::from_pairs(current, next, PreviousKey::Absent, rsa))
            })
        })
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError::DuplicateKeyId` when two of the keys share a key id.
    pub fn from_pairs(
        current: JwkPair,
        next: JwkPair,
        previous: PreviousKey<JwkPair>,
        rsa: RsaKeyRing,
    ) -> Result<Self, JwksKeyError> {
        let mut published = vec![current.public_jwk().clone()];
        let mut verification_keys = vec![current.verification_key().clone()];

        if let PreviousKey::Retired(retired) = &previous {
            published.push(retired.public_jwk().clone());
            verification_keys.push(retired.verification_key().clone());
        }

        published.push(next.public_jwk().clone());
        published.extend(rsa.published());
        verification_keys.push(next.verification_key().clone());

        let key_set = match VerificationKeySet::assemble(verification_keys) {
            KeySetAssembly::Assembled(key_set) => key_set,
            KeySetAssembly::DuplicateKeyId(duplicate) => {
                return Err(JwksKeyError::DuplicateKeyId { duplicate });
            }
        };

        Ok(Self {
            current,
            key_set,
            next,
            previous,
            public_jwks: PublicJwks::new(published),
            rsa,
        })
    }

    #[must_use]
    pub fn current(&self) -> &JwkPair {
        &self.current
    }

    #[must_use]
    pub fn key_set(&self) -> &VerificationKeySet {
        &self.key_set
    }

    #[must_use]
    pub fn next(&self) -> &JwkPair {
        &self.next
    }

    #[must_use]
    pub fn previous(&self) -> &PreviousKey<JwkPair> {
        &self.previous
    }

    #[must_use]
    pub fn public_jwks(&self) -> &PublicJwks {
        &self.public_jwks
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError` when the next key cannot be generated or the keys do not form a key set.
    pub fn rotate(&self, rsa_keys: &dyn ProvidesRsaSigningKeys) -> Result<Self, JwksKeyError> {
        random_pair(self.current.signing_key().curve()).and_then(|next| {
            self.rsa.rotate(rsa_keys).and_then(|rsa| {
                Self::from_pairs(
                    self.next.clone(),
                    next,
                    PreviousKey::Retired(Box::new(self.current.clone())),
                    rsa,
                )
            })
        })
    }

    #[must_use]
    pub fn rsa(&self) -> &RsaKeyRing {
        &self.rsa
    }

    #[must_use]
    pub fn verify_jwt<TClaims: DeserializeOwned, TProfile: JwtProfile>(
        &self,
        token: &str,
        expectation: &JwtExpectation,
        now: NumericDate,
    ) -> JwksSecretVerificationResult<TClaims, TProfile> {
        let verified = match verify_serialized_jwt(&self.key_set, token, expectation, now) {
            JwtVerification::Rejected(rejection) => {
                return JwksSecretVerificationResult::Rejected(rejection);
            }
            JwtVerification::Verified(verified) => verified,
        };

        if &verified.kid == self.current.kid() {
            JwksSecretVerificationResult::SignedWithCurrent(verified)
        } else if self.previous.is_retired_key(&verified.kid) {
            JwksSecretVerificationResult::SignedWithPrevious(verified)
        } else {
            JwksSecretVerificationResult::SignedWithNextKey
        }
    }
}

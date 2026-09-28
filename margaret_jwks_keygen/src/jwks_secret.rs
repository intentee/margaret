use serde::de::DeserializeOwned;
use uuid::Uuid;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::ec_signing_key::EcSigningKey;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret_verification_result::JwksSecretVerificationResult;
use crate::previous_key::PreviousKey;
use crate::public_jwks::PublicJwks;

fn random_pair(curve: Curve) -> Result<JwkPair, JwksKeyError> {
    EcSigningKey::generate(curve)
        .and_then(|signing_key| JwkPair::new(KeyId::new(Uuid::new_v4().to_string()), signing_key))
}

#[derive(Clone)]
pub struct JwksSecret {
    current: JwkPair,
    key_set: VerificationKeySet,
    next: JwkPair,
    previous: PreviousKey,
    public_jwks: PublicJwks,
}

impl JwksSecret {
    /// # Errors
    ///
    /// Returns `JwksKeyError` when a key cannot be generated or the keys do not form a key set.
    pub fn fresh(curve: Curve) -> Result<Self, JwksKeyError> {
        random_pair(curve).and_then(|current| {
            random_pair(curve).and_then(|next| Self::from_pairs(current, next, PreviousKey::Absent))
        })
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError::KeySetRejected` when the keys do not form a key set.
    pub fn from_pairs(
        current: JwkPair,
        next: JwkPair,
        previous: PreviousKey,
    ) -> Result<Self, JwksKeyError> {
        let mut keys = vec![current.public_jwk().clone()];

        if let PreviousKey::Retired(retired) = &previous {
            keys.push(retired.public_jwk().clone());
        }

        keys.push(next.public_jwk().clone());

        let key_set = match VerificationKeySet::from_jwks(keys.clone()) {
            KeySetParsing::Accepted(key_set) => key_set,
            KeySetParsing::Rejected(rejection) => {
                return Err(JwksKeyError::KeySetRejected { rejection });
            }
        };

        Ok(Self {
            current,
            key_set,
            next,
            previous,
            public_jwks: PublicJwks::new(keys),
        })
    }

    #[must_use]
    pub fn current(&self) -> &JwkPair {
        &self.current
    }

    #[must_use]
    pub fn next(&self) -> &JwkPair {
        &self.next
    }

    #[must_use]
    pub fn previous(&self) -> &PreviousKey {
        &self.previous
    }

    #[must_use]
    pub fn public_jwks(&self) -> &PublicJwks {
        &self.public_jwks
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError` when the next key cannot be generated or the keys do not form a key set.
    pub fn rotate(&self) -> Result<Self, JwksKeyError> {
        random_pair(self.current.signing_key().curve()).and_then(|next| {
            Self::from_pairs(
                self.next.clone(),
                next,
                PreviousKey::Retired(Box::new(self.current.clone())),
            )
        })
    }

    #[must_use]
    pub fn verify_jwt<TClaims: DeserializeOwned>(
        &self,
        token: &str,
        now: NumericDate,
    ) -> JwksSecretVerificationResult<TClaims> {
        let verified = match verify_jwt(&self.key_set, token, now) {
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

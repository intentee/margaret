use serde::Deserialize;
use serde::Serialize;
use zeroize::Zeroizing;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;

use crate::ec_signing_key::EcSigningKey;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret::JwksSecret;
use crate::previous_key::PreviousKey;

#[derive(Deserialize, Serialize)]
struct PersistedSigningKey {
    crv: Curve,
    kid: KeyId,
    pem: Zeroizing<String>,
}

#[derive(Deserialize, Serialize)]
struct PersistedPair {
    public: Jwk,
    signing: PersistedSigningKey,
}

impl PersistedPair {
    fn of(pair: &JwkPair) -> Self {
        Self {
            public: pair.public_jwk().clone(),
            signing: PersistedSigningKey {
                crv: pair.signing_key().curve(),
                kid: pair.kid().clone(),
                pem: pair.signing_key().pem().clone(),
            },
        }
    }

    fn restore(self) -> Result<JwkPair, JwksKeyError> {
        let Self {
            public: _,
            signing: PersistedSigningKey { crv, kid, pem },
        } = self;

        JwkPair::new(kid, EcSigningKey::from_pkcs8_pem(crv, pem)?)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PersistedJwksSecret {
    current: PersistedPair,
    next: PersistedPair,
    previous: PersistedPair,
}

impl PersistedJwksSecret {
    #[must_use]
    pub fn from_secret(secret: &JwksSecret) -> Self {
        let previous = match secret.previous() {
            PreviousKey::Absent => secret.current(),
            PreviousKey::Retired(retired) => retired,
        };

        Self {
            current: PersistedPair::of(secret.current()),
            next: PersistedPair::of(secret.next()),
            previous: PersistedPair::of(previous),
        }
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError` when a persisted key cannot be restored or the keys do not form a key set.
    pub fn into_secret(self) -> Result<JwksSecret, JwksKeyError> {
        let Self {
            current,
            next,
            previous,
        } = self;
        let current = current.restore()?;
        let next = next.restore()?;
        let previous = previous.restore()?;
        let previous = if previous == current {
            PreviousKey::Absent
        } else {
            PreviousKey::Retired(Box::new(previous))
        };

        JwksSecret::from_pairs(current, next, previous)
    }
}

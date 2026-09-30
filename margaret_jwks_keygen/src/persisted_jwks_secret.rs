use base64ct::Base64;
use base64ct::Encoding;
use serde::Deserialize;
use serde::Serialize;
use zeroize::Zeroizing;

use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;

use crate::ec_signing_key::EcSigningKey;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret::JwksSecret;
use crate::previous_key::PreviousKey;
use crate::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use crate::rsa_jwk_pair::RsaJwkPair;
use crate::rsa_key_ring::RsaKeyRing;
use crate::rsa_signing_key::RsaSigningKey;
use crate::signing_curve::SigningCurve;

#[derive(Deserialize, Serialize)]
struct PersistedSigningKey {
    crv: SigningCurve,
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
struct PersistedRsaPair {
    kid: KeyId,
    pkcs8: Zeroizing<String>,
}

impl PersistedRsaPair {
    fn of(pair: &RsaJwkPair) -> Self {
        Self {
            kid: pair.kid().clone(),
            pkcs8: Zeroizing::new(Base64::encode_string(pair.signing_key().pkcs8())),
        }
    }

    fn restore(self) -> Result<RsaJwkPair, JwksKeyError> {
        let Self { kid, pkcs8 } = self;
        let pkcs8 = Base64::decode_vec(&pkcs8)
            .map(Zeroizing::new)
            .map_err(|source| JwksKeyError::RsaKeyBase64 { source })?;

        RsaSigningKey::from_pkcs8(pkcs8).map(|signing_key| RsaJwkPair::new(kid, signing_key))
    }
}

#[derive(Deserialize, Serialize)]
struct PersistedRsaRing {
    current: PersistedRsaPair,
    next: PersistedRsaPair,
    previous: PersistedRsaPair,
}

impl PersistedRsaRing {
    fn of(ring: &RsaKeyRing) -> Self {
        let previous = match ring.previous() {
            PreviousKey::Absent => ring.current(),
            PreviousKey::Retired(retired) => retired,
        };

        Self {
            current: PersistedRsaPair::of(ring.current()),
            next: PersistedRsaPair::of(ring.next()),
            previous: PersistedRsaPair::of(previous),
        }
    }

    fn restore(self) -> Result<RsaKeyRing, JwksKeyError> {
        let Self {
            current,
            next,
            previous,
        } = self;
        current.restore().and_then(|current| {
            next.restore().and_then(|next| {
                previous.restore().map(|previous| {
                    let previous = if previous.kid() == current.kid() {
                        PreviousKey::Absent
                    } else {
                        PreviousKey::Retired(Box::new(previous))
                    };

                    RsaKeyRing::new(current, next, previous)
                })
            })
        })
    }
}

#[derive(Deserialize, Serialize)]
struct PersistedEcKeys {
    current: PersistedPair,
    next: PersistedPair,
    previous: PersistedPair,
}

impl PersistedEcKeys {
    fn of(secret: &JwksSecret) -> Self {
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

    fn into_secret(self, rsa: RsaKeyRing) -> Result<JwksSecret, JwksKeyError> {
        let Self {
            current,
            next,
            previous,
        } = self;
        let current = current.restore()?;
        let next = next.restore()?;
        let previous = previous.restore()?;
        let previous =
            if previous.kid() == current.kid() && previous.signing_key() == current.signing_key() {
                PreviousKey::Absent
            } else {
                PreviousKey::Retired(Box::new(previous))
            };

        JwksSecret::from_pairs(current, next, previous, rsa)
    }
}

#[derive(Deserialize, Serialize)]
struct PersistedKeys {
    #[serde(flatten)]
    ec: PersistedEcKeys,
    rsa: PersistedRsaRing,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum PersistedDocument {
    WithRsaKeys(PersistedKeys),
    WithoutRsaKeys(PersistedEcKeys),
}

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersistedJwksSecret {
    document: PersistedDocument,
}

impl PersistedJwksSecret {
    #[must_use]
    pub fn from_secret(secret: &JwksSecret) -> Self {
        Self {
            document: PersistedDocument::WithRsaKeys(PersistedKeys {
                ec: PersistedEcKeys::of(secret),
                rsa: PersistedRsaRing::of(secret.rsa()),
            }),
        }
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError` when a persisted key cannot be restored, the rsa keys a document
    /// persisted before they existed cannot be provided, or the keys do not form a key set.
    pub fn into_secret(
        self,
        rsa_keys: &dyn ProvidesRsaSigningKeys,
    ) -> Result<JwksSecret, JwksKeyError> {
        match self.document {
            PersistedDocument::WithRsaKeys(PersistedKeys { ec, rsa }) => {
                rsa.restore().and_then(|rsa| ec.into_secret(rsa))
            }
            PersistedDocument::WithoutRsaKeys(ec) => {
                RsaKeyRing::fresh(rsa_keys).and_then(|rsa| ec.into_secret(rsa))
            }
        }
    }
}

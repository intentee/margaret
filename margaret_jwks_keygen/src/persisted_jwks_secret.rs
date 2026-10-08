use base64ct::Base64;
use base64ct::Encoding;
use serde::Deserialize;
use serde::Serialize;
use zeroize::Zeroizing;

use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::ec_signing_key::EcSigningKey;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret::JwksSecret;
use crate::jwks_secret_parts::JwksSecretParts;
use crate::key_retention::KeyRetention;
use crate::retired_key::RetiredKey;
use crate::rsa_jwk_pair::RsaJwkPair;
use crate::rsa_key_ring::RsaKeyRing;
use crate::rsa_signing_key::RsaSigningKey;
use crate::signing_curve::SigningCurve;
use crate::signing_keys_generation::SigningKeysGeneration;

fn persisted_retired(retired: &[RetiredKey]) -> Vec<PersistedRetiredKey> {
    retired
        .iter()
        .map(|retired| PersistedRetiredKey {
            public: retired.public_jwk().clone(),
            retired_at: retired.retired_at(),
        })
        .collect()
}

fn restored_retired(retired: Vec<PersistedRetiredKey>) -> Result<Vec<RetiredKey>, JwksKeyError> {
    retired
        .into_iter()
        .map(|PersistedRetiredKey { public, retired_at }| RetiredKey::restore(public, retired_at))
        .collect()
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PersistedEcKey {
    crv: SigningCurve,
    kid: KeyId,
    pem: Zeroizing<String>,
}

impl PersistedEcKey {
    fn of(pair: &JwkPair) -> Self {
        Self {
            crv: pair.signing_key().curve(),
            kid: pair.kid().clone(),
            pem: pair.signing_key().pem().clone(),
        }
    }

    fn restore(self) -> Result<JwkPair, JwksKeyError> {
        let Self { crv, kid, pem } = self;

        JwkPair::new(kid, EcSigningKey::from_pkcs8_pem(crv, pem)?)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PersistedRsaKey {
    kid: KeyId,
    pkcs8: Zeroizing<String>,
}

impl PersistedRsaKey {
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

        RsaSigningKey::from_pkcs8(pkcs8).and_then(|signing_key| RsaJwkPair::new(kid, signing_key))
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PersistedRetiredKey {
    public: Jwk,
    retired_at: NumericDate,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PersistedEcKeys {
    current: PersistedEcKey,
    next: PersistedEcKey,
    retired: Vec<PersistedRetiredKey>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PersistedRsaKeys {
    current: PersistedRsaKey,
    next: PersistedRsaKey,
    retired: Vec<PersistedRetiredKey>,
}

impl PersistedRsaKeys {
    fn restore(self) -> Result<RsaKeyRing, JwksKeyError> {
        let Self {
            current,
            next,
            retired,
        } = self;

        Ok(RsaKeyRing::new(
            current.restore()?,
            next.restore()?,
            restored_retired(retired)?,
        ))
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedJwksSecret {
    ec: PersistedEcKeys,
    rolled_at: NumericDate,
    rsa: PersistedRsaKeys,
}

impl PersistedJwksSecret {
    #[must_use]
    pub fn from_secret(secret: &JwksSecret) -> Self {
        Self {
            ec: PersistedEcKeys {
                current: PersistedEcKey::of(secret.current()),
                next: PersistedEcKey::of(secret.next()),
                retired: persisted_retired(secret.retired()),
            },
            rolled_at: secret.rolled_at(),
            rsa: PersistedRsaKeys {
                current: PersistedRsaKey::of(secret.rsa().current()),
                next: PersistedRsaKey::of(secret.rsa().next()),
                retired: persisted_retired(secret.rsa().retired()),
            },
        }
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError` when a persisted key cannot be restored or the keys do not form a
    /// key set.
    pub fn into_secret(
        self,
        generation: SigningKeysGeneration,
        retention: KeyRetention,
    ) -> Result<JwksSecret, JwksKeyError> {
        let Self { ec, rolled_at, rsa } = self;
        let PersistedEcKeys {
            current,
            next,
            retired,
        } = ec;

        JwksSecret::assembled(JwksSecretParts {
            current: current.restore()?,
            generation,
            next: next.restore()?,
            retention,
            retired: restored_retired(retired)?,
            rolled_at,
            rsa: rsa.restore()?,
        })
    }
}

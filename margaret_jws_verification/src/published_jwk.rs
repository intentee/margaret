use std::ops::ControlFlow;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Map;
use serde_json::Value;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_use::KeyUse;

use crate::ec_jwk::EcJwk;
use crate::ignored_key_reason::IgnoredKeyReason;
use crate::jwk::Jwk;
use crate::key_id::KeyId;
use crate::key_type::KeyType;
use crate::parameter_value::ParameterValue;
use crate::published_ec_members::PublishedEcMembers;
use crate::published_rsa_members::PublishedRsaMembers;
use crate::rsa_jwk::RsaJwk;

fn typed_members<TMembers: DeserializeOwned>(
    members: Map<String, Value>,
) -> ControlFlow<IgnoredKeyReason, TMembers> {
    match serde_json::from_value(Value::Object(members)) {
        Ok(typed) => ControlFlow::Continue(typed),
        Err(source) => ControlFlow::Break(IgnoredKeyReason::Malformed { source }),
    }
}

#[derive(Deserialize)]
pub(crate) struct PublishedJwk {
    #[serde(default)]
    alg: Option<ParameterValue<JwsAlgorithm>>,
    #[serde(default)]
    kid: Option<KeyId>,
    #[serde(default, rename = "use")]
    key_use: Option<ParameterValue<KeyUse>>,
    kty: ParameterValue<KeyType>,
    #[serde(flatten)]
    members: Map<String, Value>,
}

impl PublishedJwk {
    pub(crate) fn into_jwk(self) -> ControlFlow<IgnoredKeyReason, Jwk> {
        let Self {
            alg,
            kid,
            key_use,
            kty,
            members,
        } = self;
        let key_type = kty.supported(|kty| IgnoredKeyReason::UnsupportedKeyType { kty })?;
        let alg = ParameterValue::supported_when_present(alg, |alg| {
            IgnoredKeyReason::UnsupportedAlgorithm { alg }
        })?;
        let key_use = ParameterValue::supported_when_present(key_use, |key_use| {
            IgnoredKeyReason::UnrecognizedUse { key_use }
        })?;

        match key_type {
            KeyType::Ec => {
                let PublishedEcMembers { crv, x, y } = typed_members(members)?;

                ControlFlow::Continue(Jwk::Ec(EcJwk {
                    alg,
                    crv: crv.supported(|crv| IgnoredKeyReason::UnsupportedCurve { crv })?,
                    kid,
                    key_use,
                    x,
                    y,
                }))
            }
            KeyType::Rsa => {
                let PublishedRsaMembers { e, n } = typed_members(members)?;

                ControlFlow::Continue(Jwk::Rsa(RsaJwk {
                    alg,
                    e,
                    kid,
                    key_use,
                    n,
                }))
            }
        }
    }
}

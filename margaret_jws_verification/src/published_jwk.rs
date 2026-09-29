use std::ops::ControlFlow;

use serde::Deserialize;
use serde_json::Map;
use serde_json::Value;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_operation::KeyOperation;
use margaret_jose_parameters::key_use::KeyUse;

use crate::declared_algorithm::confirm_declared_algorithm;
use crate::ignored_key_reason::IgnoredKeyReason;
use crate::key_disclosure::KeyDisclosure;
use crate::key_exclusion::KeyExclusion;
use crate::key_id::KeyId;
use crate::key_set_composition::KeySetComposition;
use crate::key_type::KeyType;
use crate::parameter_value::ParameterValue;
use crate::private_key_members::PrivateKeyMembers;
use crate::published_certificate::PublishedCertificate;
use crate::published_public_key::PublishedPublicKey;
use crate::verification_key::VerificationKey;
use crate::verification_usage::verification_usage;

#[derive(Deserialize)]
pub(crate) struct PublishedJwk {
    #[serde(default)]
    alg: Option<ParameterValue<JwsAlgorithm>>,
    #[serde(default)]
    key_ops: Option<Vec<ParameterValue<KeyOperation>>>,
    #[serde(default)]
    kid: Option<KeyId>,
    #[serde(default, rename = "use")]
    key_use: Option<ParameterValue<KeyUse>>,
    kty: ParameterValue<KeyType>,
    #[serde(default)]
    x5c: Option<Vec<String>>,
    #[serde(default)]
    x5t: Option<String>,
    #[serde(default, rename = "x5t#S256")]
    x5t_s256: Option<String>,
    #[serde(flatten)]
    private_members: PrivateKeyMembers,
    #[serde(flatten)]
    members: Map<String, Value>,
}

impl PublishedJwk {
    pub(crate) fn declares_encryption(&self) -> bool {
        matches!(
            self.key_use,
            Some(ParameterValue::Supported(KeyUse::Encryption))
        ) || self.key_ops.iter().flatten().any(|key_op| {
            matches!(key_op, ParameterValue::Supported(key_op) if key_op.key_use() == KeyUse::Encryption)
        })
    }

    pub(crate) fn into_verification_key(
        self,
        composition: KeySetComposition,
    ) -> ControlFlow<KeyExclusion, VerificationKey> {
        let Self {
            alg,
            key_ops,
            kid,
            key_use,
            kty,
            x5c,
            x5t,
            x5t_s256,
            private_members,
            members,
        } = self;

        if private_members.discloses_private_key() {
            return ControlFlow::Break(KeyExclusion::Disclosed(KeyDisclosure::PrivateKey));
        }

        let public_key = match kty {
            ParameterValue::Supported(KeyType::Ec) => PublishedPublicKey::Ec(members),
            ParameterValue::Supported(KeyType::Oct) => {
                return ControlFlow::Break(KeyExclusion::Disclosed(KeyDisclosure::SymmetricKey));
            }
            ParameterValue::Supported(KeyType::Rsa) => PublishedPublicKey::Rsa(members),
            ParameterValue::Unsupported(kty) => {
                return ControlFlow::Break(KeyExclusion::Ignored(
                    IgnoredKeyReason::UnsupportedKeyType { kty },
                ));
            }
        };

        verification_usage(key_use, key_ops, composition)
            .map_break(|rejection| KeyExclusion::Ignored(IgnoredKeyReason::Usage(rejection)))?;

        let Some(kid) = kid else {
            return ControlFlow::Break(KeyExclusion::Ignored(IgnoredKeyReason::MissingKeyId));
        };
        let material = public_key
            .into_material()
            .map_break(|rejection| KeyExclusion::Ignored(IgnoredKeyReason::Material(rejection)))?;

        confirm_declared_algorithm(alg, &material)
            .map_break(|rejection| KeyExclusion::Ignored(IgnoredKeyReason::Algorithm(rejection)))?;

        if let Some(chain) = x5c {
            PublishedCertificate {
                chain,
                sha1_thumbprint: x5t,
                sha256_thumbprint: x5t_s256,
            }
            .attests(&material)
            .map_break(|rejection| {
                KeyExclusion::Ignored(IgnoredKeyReason::Certificate(rejection))
            })?;
        }

        ControlFlow::Continue(VerificationKey::new(kid, material))
    }
}

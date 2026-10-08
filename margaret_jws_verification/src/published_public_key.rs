use std::ops::ControlFlow;

use serde::de::DeserializeOwned;
use serde_json::Map;
use serde_json::Value;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::confirmed_material::confirmed_material;
use crate::ignored_key_reason::IgnoredKeyReason;
use crate::key_material_rejection::KeyMaterialRejection;
use crate::parameter_value::ParameterValue;
use crate::published_ec_members::PublishedEcMembers;
use crate::published_okp_members::PublishedOkpMembers;
use crate::published_rsa_members::PublishedRsaMembers;
use crate::rsa_material::rsa_material;
use crate::verification_material::VerificationMaterial;

fn typed_members<TMembers: DeserializeOwned>(
    members: Map<String, Value>,
) -> ControlFlow<IgnoredKeyReason, TMembers> {
    match serde_json::from_value(Value::Object(members)) {
        Ok(typed) => ControlFlow::Continue(typed),
        Err(source) => ControlFlow::Break(IgnoredKeyReason::Material(
            KeyMaterialRejection::MalformedMembers { source },
        )),
    }
}

fn ec_material(
    PublishedEcMembers { crv, x, y }: PublishedEcMembers,
) -> ControlFlow<KeyMaterialRejection, VerificationMaterial> {
    let crv = crv.supported(|crv| KeyMaterialRejection::UnsupportedCurve { crv })?;

    VerificationMaterial::from_coordinates(crv, &x, &y)
}

fn octet_key_pair_material(
    PublishedOkpMembers { crv, x }: PublishedOkpMembers,
) -> ControlFlow<KeyMaterialRejection, VerificationMaterial> {
    let crv = crv.supported(|crv| KeyMaterialRejection::UnsupportedOctetKeyPairCurve { crv })?;

    VerificationMaterial::from_octet_key_pair(crv, &x)
}

pub(crate) enum PublishedPublicKey {
    Ec(Map<String, Value>),
    OctetKeyPair(Map<String, Value>),
    Rsa(Map<String, Value>),
}

impl PublishedPublicKey {
    pub(crate) fn into_material(
        self,
        alg: Option<ParameterValue<JwsAlgorithm>>,
    ) -> ControlFlow<IgnoredKeyReason, VerificationMaterial> {
        match self {
            Self::Ec(members) => confirmed_material(ec_material(typed_members(members)?), alg),
            Self::OctetKeyPair(members) => {
                confirmed_material(octet_key_pair_material(typed_members(members)?), alg)
            }
            Self::Rsa(members) => {
                let PublishedRsaMembers { e, n } = typed_members(members)?;

                rsa_material(&n, &e, alg)
            }
        }
    }
}

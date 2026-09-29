use std::ops::ControlFlow;

use serde::de::DeserializeOwned;
use serde_json::Map;
use serde_json::Value;

use crate::key_material_rejection::KeyMaterialRejection;
use crate::published_ec_members::PublishedEcMembers;
use crate::published_rsa_members::PublishedRsaMembers;
use crate::verification_material::VerificationMaterial;

fn typed_members<TMembers: DeserializeOwned>(
    members: Map<String, Value>,
) -> ControlFlow<KeyMaterialRejection, TMembers> {
    match serde_json::from_value(Value::Object(members)) {
        Ok(typed) => ControlFlow::Continue(typed),
        Err(source) => ControlFlow::Break(KeyMaterialRejection::MalformedMembers { source }),
    }
}

pub(crate) enum PublishedPublicKey {
    Ec(Map<String, Value>),
    Rsa(Map<String, Value>),
}

impl PublishedPublicKey {
    pub(crate) fn into_material(self) -> ControlFlow<KeyMaterialRejection, VerificationMaterial> {
        match self {
            Self::Ec(members) => {
                let PublishedEcMembers { crv, x, y } = typed_members(members)?;

                VerificationMaterial::from_coordinates(
                    crv.supported(|crv| KeyMaterialRejection::UnsupportedCurve { crv })?,
                    &x,
                    &y,
                )
            }
            Self::Rsa(members) => {
                let PublishedRsaMembers { e, n } = typed_members(members)?;

                VerificationMaterial::from_rsa_components(&n, &e)
            }
        }
    }
}

use std::ops::ControlFlow;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::declared_algorithm::confirm_declared_algorithm;
use crate::ignored_key_reason::IgnoredKeyReason;
use crate::key_material_rejection::KeyMaterialRejection;
use crate::parameter_value::ParameterValue;
use crate::verification_material::VerificationMaterial;

pub(crate) fn confirmed_material(
    material: ControlFlow<KeyMaterialRejection, VerificationMaterial>,
    alg: Option<ParameterValue<JwsAlgorithm>>,
) -> ControlFlow<IgnoredKeyReason, VerificationMaterial> {
    let material = material.map_break(IgnoredKeyReason::Material)?;

    confirm_declared_algorithm(alg, &material).map_break(IgnoredKeyReason::Algorithm)?;

    ControlFlow::Continue(material)
}

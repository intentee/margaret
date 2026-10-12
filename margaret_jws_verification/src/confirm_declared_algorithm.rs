use std::ops::ControlFlow;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::key_algorithm_rejection::KeyAlgorithmRejection;
use crate::parameter_value::ParameterValue;
use crate::verification_material::VerificationMaterial;

pub(crate) fn confirm_declared_algorithm(
    alg: Option<ParameterValue<JwsAlgorithm>>,
    material: &VerificationMaterial,
) -> ControlFlow<KeyAlgorithmRejection> {
    match alg {
        Some(ParameterValue::Supported(declared)) if !material.admits(declared) => {
            ControlFlow::Break(KeyAlgorithmRejection::AlgorithmMismatch {
                declared,
                implied: material.algorithm(),
            })
        }
        Some(ParameterValue::Unsupported(alg)) => {
            ControlFlow::Break(KeyAlgorithmRejection::UnsupportedAlgorithm { alg })
        }
        Some(ParameterValue::Supported(_)) | None => ControlFlow::Continue(()),
    }
}

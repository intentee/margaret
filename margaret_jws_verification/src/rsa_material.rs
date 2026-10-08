use std::ops::ControlFlow;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::declared_rsa_scheme::declared_rsa_scheme;
use crate::ignored_key_reason::IgnoredKeyReason;
use crate::parameter_value::ParameterValue;
use crate::verification_material::VerificationMaterial;

pub(crate) fn rsa_material(
    n: &str,
    e: &str,
    alg: Option<ParameterValue<JwsAlgorithm>>,
) -> ControlFlow<IgnoredKeyReason, VerificationMaterial> {
    let scheme = declared_rsa_scheme(alg).map_break(IgnoredKeyReason::Algorithm)?;

    VerificationMaterial::from_rsa_components(n, e, scheme).map_break(IgnoredKeyReason::Material)
}

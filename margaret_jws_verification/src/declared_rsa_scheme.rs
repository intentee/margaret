use std::ops::ControlFlow;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::key_algorithm_rejection::KeyAlgorithmRejection;
use crate::parameter_value::ParameterValue;
use crate::rsa_signature_scheme::RsaSignatureScheme;
use crate::signature_scheme::SignatureScheme;

pub(crate) fn declared_rsa_scheme(
    alg: Option<ParameterValue<JwsAlgorithm>>,
) -> ControlFlow<KeyAlgorithmRejection, RsaSignatureScheme> {
    match alg {
        None => ControlFlow::Continue(RsaSignatureScheme::Pkcs1Sha256),
        Some(ParameterValue::Supported(declared)) => match SignatureScheme::of(declared) {
            SignatureScheme::Rsa(scheme) => ControlFlow::Continue(scheme),
            SignatureScheme::Ecdsa(_) | SignatureScheme::Ed25519 => {
                ControlFlow::Break(KeyAlgorithmRejection::NotAnRsaAlgorithm { declared })
            }
        },
        Some(ParameterValue::Unsupported(alg)) => {
            ControlFlow::Break(KeyAlgorithmRejection::UnsupportedAlgorithm { alg })
        }
    }
}

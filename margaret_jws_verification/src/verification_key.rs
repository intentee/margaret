use std::ops::ControlFlow;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_use::KeyUse;

use crate::ec_jwk::EcJwk;
use crate::jwk::Jwk;
use crate::jwk_rejection::JwkRejection;
use crate::key_id::KeyId;
use crate::verification_material::VerificationMaterial;

#[derive(Clone)]
pub(crate) struct VerificationKey {
    pub(crate) algorithm: JwsAlgorithm,
    pub(crate) kid: KeyId,
    pub(crate) material: VerificationMaterial,
}

impl VerificationKey {
    pub(crate) fn from_jwk(jwk: Jwk) -> ControlFlow<JwkRejection, Self> {
        let Jwk::Ec(EcJwk {
            alg,
            crv,
            kid,
            key_use,
            x,
            y,
        }) = jwk;

        if key_use == Some(KeyUse::Encryption) {
            return ControlFlow::Break(JwkRejection::UnsupportedUse);
        }

        let Some(kid) = kid else {
            return ControlFlow::Break(JwkRejection::MissingKeyId);
        };
        let algorithm = crv.algorithm();

        if let Some(declared) = alg
            && declared != algorithm
        {
            return ControlFlow::Break(JwkRejection::AlgorithmMismatch {
                algorithm: declared,
                curve: crv,
            });
        }

        ControlFlow::Continue(Self {
            algorithm,
            kid,
            material: VerificationMaterial::from_coordinates(crv, &x, &y)?,
        })
    }
}

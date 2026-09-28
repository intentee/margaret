use std::ops::ControlFlow;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_use::KeyUse;

use crate::ec_jwk::EcJwk;
use crate::jwk::Jwk;
use crate::jwk_rejection::JwkRejection;
use crate::key_id::KeyId;
use crate::rsa_jwk::RsaJwk;
use crate::verification_material::VerificationMaterial;

#[derive(Clone)]
pub(crate) struct VerificationKey {
    pub(crate) algorithm: JwsAlgorithm,
    pub(crate) kid: KeyId,
    pub(crate) material: VerificationMaterial,
}

fn signing_key_id(key_use: Option<KeyUse>, kid: Option<KeyId>) -> ControlFlow<JwkRejection, KeyId> {
    if key_use == Some(KeyUse::Encryption) {
        return ControlFlow::Break(JwkRejection::UnsupportedUse);
    }

    match kid {
        Some(kid) => ControlFlow::Continue(kid),
        None => ControlFlow::Break(JwkRejection::MissingKeyId),
    }
}

fn implied_algorithm(
    declared: Option<JwsAlgorithm>,
    implied: JwsAlgorithm,
) -> ControlFlow<JwkRejection, JwsAlgorithm> {
    match declared {
        Some(declared) if declared != implied => {
            ControlFlow::Break(JwkRejection::AlgorithmMismatch { declared, implied })
        }
        _ => ControlFlow::Continue(implied),
    }
}

impl VerificationKey {
    pub(crate) fn from_jwk(jwk: Jwk) -> ControlFlow<JwkRejection, Self> {
        match jwk {
            Jwk::Ec(EcJwk {
                alg,
                crv,
                kid,
                key_use,
                x,
                y,
            }) => ControlFlow::Continue(Self {
                kid: signing_key_id(key_use, kid)?,
                algorithm: implied_algorithm(alg, crv.algorithm())?,
                material: VerificationMaterial::from_coordinates(crv, &x, &y)?,
            }),
            Jwk::Rsa(RsaJwk {
                alg,
                e,
                kid,
                key_use,
                n,
            }) => ControlFlow::Continue(Self {
                kid: signing_key_id(key_use, kid)?,
                algorithm: implied_algorithm(alg, JwsAlgorithm::Rs256)?,
                material: VerificationMaterial::from_rsa_components(&n, &e)?,
            }),
        }
    }
}

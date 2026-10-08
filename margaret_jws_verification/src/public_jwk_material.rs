use std::ops::ControlFlow;

use crate::confirmed_material::confirmed_material;
use crate::ec_jwk::EcJwk;
use crate::ignored_key_reason::IgnoredKeyReason;
use crate::jwk::Jwk;
use crate::parameter_value::ParameterValue;
use crate::rsa_jwk::RsaJwk;
use crate::rsa_material::rsa_material;
use crate::verification_material::VerificationMaterial;

pub fn public_jwk_material(jwk: &Jwk) -> ControlFlow<IgnoredKeyReason, VerificationMaterial> {
    match jwk {
        Jwk::Ec(EcJwk { alg, crv, x, y, .. }) => confirmed_material(
            VerificationMaterial::from_coordinates(*crv, x, y),
            alg.map(ParameterValue::Supported),
        ),
        Jwk::Rsa(RsaJwk { alg, e, n, .. }) => {
            rsa_material(n, e, alg.map(ParameterValue::Supported))
        }
    }
}

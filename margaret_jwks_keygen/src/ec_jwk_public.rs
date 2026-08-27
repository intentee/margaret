use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::de::DeserializeOwned;

use crate::checks_signature::ChecksSignature;
use crate::curve::Curve;
use crate::jwks_key_error::JwksKeyError;
use crate::jws_algorithm::JwsAlgorithm;
use crate::key_use::KeyUse;
use crate::signature_check::SignatureCheck;
use crate::token_verification::TokenVerification;
use crate::verifies_token::VerifiesToken;
use crate::verify_compact_jws::verify_compact_jws;
use crate::verify_ec_signature::verify_ec_signature;

const EC_KEY_TYPE: &str = "EC";

#[derive(Clone, Debug, Deserialize)]
pub struct EcJwkPublic {
    pub crv: Curve,
    pub kid: String,
    #[serde(rename = "use")]
    pub use_: KeyUse,
    pub x: String,
    pub y: String,
}

impl ChecksSignature for EcJwkPublic {
    fn algorithm(&self) -> JwsAlgorithm {
        self.crv.algorithm()
    }

    fn check_signature(
        &self,
        signing_input: &str,
        signature_bytes: &[u8],
    ) -> Result<SignatureCheck, JwksKeyError> {
        verify_ec_signature(signing_input, signature_bytes, &self.x, &self.y, self.crv)
    }
}

impl Serialize for EcJwkPublic {
    fn serialize<Target>(&self, serializer: Target) -> Result<Target::Ok, Target::Error>
    where
        Target: Serializer,
    {
        #[derive(Serialize)]
        struct EcJwkPublicWire<'key> {
            alg: &'key str,
            crv: Curve,
            kid: &'key str,
            kty: &'key str,
            #[serde(rename = "use")]
            use_: &'key KeyUse,
            x: &'key str,
            y: &'key str,
        }

        EcJwkPublicWire {
            alg: self.algorithm().wire_name(),
            crv: self.crv,
            kid: &self.kid,
            kty: EC_KEY_TYPE,
            use_: &self.use_,
            x: &self.x,
            y: &self.y,
        }
        .serialize(serializer)
    }
}

impl VerifiesToken for EcJwkPublic {
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError> {
        verify_compact_jws(self, token)
    }
}

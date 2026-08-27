use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::de::DeserializeOwned;

use crate::checks_signature::ChecksSignature;
use crate::jwks_key_error::JwksKeyError;
use crate::jws_algorithm::JwsAlgorithm;
use crate::key_use::KeyUse;
use crate::signature_check::SignatureCheck;
use crate::token_verification::TokenVerification;
use crate::verifies_token::VerifiesToken;
use crate::verify_compact_jws::verify_compact_jws;
use crate::verify_rsa_signature::verify_rsa_signature;

const RSA_KEY_TYPE: &str = "RSA";

#[derive(Clone, Debug, Deserialize)]
pub struct RsaJwkPublic {
    pub e: String,
    pub kid: String,
    pub n: String,
    #[serde(rename = "use")]
    pub use_: KeyUse,
}

impl ChecksSignature for RsaJwkPublic {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Rs256
    }

    fn check_signature(
        &self,
        signing_input: &str,
        signature_bytes: &[u8],
    ) -> Result<SignatureCheck, JwksKeyError> {
        verify_rsa_signature(signing_input, signature_bytes, &self.n, &self.e)
    }
}

impl Serialize for RsaJwkPublic {
    fn serialize<Target>(&self, serializer: Target) -> Result<Target::Ok, Target::Error>
    where
        Target: Serializer,
    {
        #[derive(Serialize)]
        struct RsaJwkPublicWire<'key> {
            alg: &'key str,
            e: &'key str,
            kid: &'key str,
            kty: &'key str,
            n: &'key str,
            #[serde(rename = "use")]
            use_: &'key KeyUse,
        }

        RsaJwkPublicWire {
            alg: self.algorithm().wire_name(),
            e: &self.e,
            kid: &self.kid,
            kty: RSA_KEY_TYPE,
            n: &self.n,
            use_: &self.use_,
        }
        .serialize(serializer)
    }
}

impl VerifiesToken for RsaJwkPublic {
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError> {
        verify_compact_jws(self, token)
    }
}

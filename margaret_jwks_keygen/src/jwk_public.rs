use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::de::DeserializeOwned;

use crate::compact_jws::CompactJws;
use crate::curve::Curve;
use crate::jwks_key_error::JwksKeyError;
use crate::jws_algorithm::JwsAlgorithm;
use crate::jws_header::JwsHeader;
use crate::key_type::KeyType;
use crate::key_use::KeyUse;
use crate::signature_check::SignatureCheck;
use crate::token_malformation::TokenMalformation;
use crate::token_verification::TokenVerification;
use crate::verifies_token::VerifiesToken;
use crate::verify_signature::verify_signature;

#[derive(Clone, Debug, Deserialize)]
pub struct JwkPublic {
    pub crv: Curve,
    pub kid: String,
    pub kty: KeyType,
    #[serde(rename = "use")]
    pub use_: KeyUse,
    pub x: String,
    pub y: String,
}

impl Serialize for JwkPublic {
    fn serialize<Target>(&self, serializer: Target) -> Result<Target::Ok, Target::Error>
    where
        Target: Serializer,
    {
        #[derive(Serialize)]
        struct JwkPublicWire<'key> {
            alg: JwsAlgorithm,
            crv: Curve,
            kid: &'key str,
            kty: &'key KeyType,
            #[serde(rename = "use")]
            use_: &'key KeyUse,
            x: &'key str,
            y: &'key str,
        }

        JwkPublicWire {
            alg: self.crv.algorithm(),
            crv: self.crv,
            kid: &self.kid,
            kty: &self.kty,
            use_: &self.use_,
            x: &self.x,
            y: &self.y,
        }
        .serialize(serializer)
    }
}

impl VerifiesToken for JwkPublic {
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError> {
        let compact_jws = match CompactJws::parse(token) {
            Ok(compact_jws) => compact_jws,
            Err(malformation) => return Ok(TokenVerification::Malformed(malformation)),
        };
        let header: JwsHeader = match serde_json::from_slice(&compact_jws.header_bytes) {
            Ok(header) => header,
            Err(source) => {
                return Ok(TokenVerification::Malformed(TokenMalformation::HeaderJson(
                    source,
                )));
            }
        };
        let expected = self.crv.algorithm();

        if header.alg != expected {
            return Ok(TokenVerification::Malformed(
                TokenMalformation::AlgorithmMismatch {
                    expected,
                    found: header.alg,
                },
            ));
        }

        match verify_signature(
            compact_jws.signing_input,
            &compact_jws.signature_bytes,
            &self.x,
            &self.y,
            self.crv,
        )? {
            SignatureCheck::Malformed(malformation) => {
                Ok(TokenVerification::Malformed(malformation))
            }
            SignatureCheck::Matches => match serde_json::from_slice(&compact_jws.payload_bytes) {
                Ok(claims) => Ok(TokenVerification::Verified(claims)),
                Err(source) => Ok(TokenVerification::Malformed(TokenMalformation::ClaimsJson(
                    source,
                ))),
            },
            SignatureCheck::Mismatch => Ok(TokenVerification::SignatureMismatch),
        }
    }
}

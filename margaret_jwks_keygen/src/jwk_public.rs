use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::compact_jws::CompactJws;
use crate::curve::Curve;
use crate::jwks_key_error::JwksKeyError;
use crate::jws_header::JwsHeader;
use crate::key_type::KeyType;
use crate::key_use::KeyUse;
use crate::token_verification::TokenVerification;
use crate::verifies_token::VerifiesToken;
use crate::verify_signature::verify_signature;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JwkPublic {
    pub crv: Curve,
    pub kid: String,
    pub kty: KeyType,
    #[serde(rename = "use")]
    pub use_: KeyUse,
    pub x: String,
    pub y: String,
}

impl VerifiesToken for JwkPublic {
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError> {
        let compact_jws = CompactJws::parse(token)?;
        let header: JwsHeader = serde_json::from_slice(&compact_jws.header_bytes)
            .map_err(|source| JwksKeyError::HeaderJson { source })?;

        if header.alg != self.crv.algorithm() {
            return Err(JwksKeyError::AlgorithmMismatch {
                expected: self.crv.algorithm(),
                found: header.alg,
            });
        }

        if !verify_signature(
            compact_jws.signing_input,
            &compact_jws.signature_bytes,
            &self.x,
            &self.y,
            self.crv,
        )? {
            return Ok(TokenVerification::SignatureMismatch);
        }

        let claims = serde_json::from_slice(&compact_jws.payload_bytes)
            .map_err(|source| JwksKeyError::ClaimsJson { source })?;

        Ok(TokenVerification::Verified(claims))
    }
}

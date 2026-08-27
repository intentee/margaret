use serde::de::DeserializeOwned;

use crate::checks_signature::ChecksSignature;
use crate::compact_jws::CompactJws;
use crate::jwks_key_error::JwksKeyError;
use crate::jws_header::JwsHeader;
use crate::signature_check::SignatureCheck;
use crate::token_malformation::TokenMalformation;
use crate::token_verification::TokenVerification;

pub(crate) fn verify_compact_jws<TClaims, TKey>(
    key: &TKey,
    token: &str,
) -> Result<TokenVerification<TClaims>, JwksKeyError>
where
    TClaims: DeserializeOwned,
    TKey: ChecksSignature,
{
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
    let expected = key.algorithm();

    if header.alg != expected {
        return Ok(TokenVerification::Malformed(
            TokenMalformation::AlgorithmMismatch {
                expected,
                found: header.alg,
            },
        ));
    }

    match key.check_signature(compact_jws.signing_input, &compact_jws.signature_bytes)? {
        SignatureCheck::Malformed(malformation) => Ok(TokenVerification::Malformed(malformation)),
        SignatureCheck::Matches => match serde_json::from_slice(&compact_jws.payload_bytes) {
            Ok(claims) => Ok(TokenVerification::Verified(claims)),
            Err(source) => Ok(TokenVerification::Malformed(TokenMalformation::ClaimsJson(
                source,
            ))),
        },
        SignatureCheck::Mismatch => Ok(TokenVerification::SignatureMismatch),
    }
}

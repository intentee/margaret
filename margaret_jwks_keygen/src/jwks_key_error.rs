use thiserror::Error;

use crate::jws_algorithm::JwsAlgorithm;

#[derive(Debug, Error)]
pub enum JwksKeyError {
    #[error("the token algorithm {found:?} does not match the key algorithm {expected:?}")]
    AlgorithmMismatch {
        expected: JwsAlgorithm,
        found: JwsAlgorithm,
    },

    #[error("the token payload segment is not valid base64url: {source}")]
    ClaimsBase64 {
        #[source]
        source: base64ct::Error,
    },

    #[error("the token payload could not be handled as json: {source}")]
    ClaimsJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("the jwk public key coordinate is not valid base64url: {source}")]
    CoordinateBase64 {
        #[source]
        source: base64ct::Error,
    },

    #[error(
        "the jwk public key coordinate decodes to {found} bytes but the curve requires exactly {expected}"
    )]
    CoordinateLength { expected: usize, found: usize },

    #[error("the token header segment is not valid base64url: {source}")]
    HeaderBase64 {
        #[source]
        source: base64ct::Error,
    },

    #[error("the token header could not be handled as json: {source}")]
    HeaderJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("the token is not a well-formed compact jws")]
    MalformedCompactJws,

    #[error("the jwk public key coordinates could not be built into a verifying key: {source}")]
    MalformedVerifyingKey {
        #[source]
        source: p256::ecdsa::Error,
    },

    #[error("the generated public key is missing its {coordinate} coordinate")]
    MissingPublicKeyCoordinate { coordinate: &'static str },

    #[error("the token signature is not in canonical low-s form")]
    NonCanonicalSignature,

    #[error("the generated private key could not be encoded to pkcs#8 pem: {source}")]
    PemEncoding {
        #[from]
        source: p256::pkcs8::Error,
    },

    #[error("the token signature segment is not valid base64url: {source}")]
    SignatureBase64 {
        #[source]
        source: base64ct::Error,
    },

    #[error("the token signature is not a valid ecdsa signature: {source}")]
    SignatureMalformed {
        #[source]
        source: p256::ecdsa::Error,
    },

    #[error("the token signature does not match the key")]
    SignatureMismatch,

    #[error("the signing key pem could not be parsed into an ecdsa signing key: {source}")]
    SigningKeyRejected {
        #[source]
        source: p256::pkcs8::Error,
    },

    #[error("no jwk in the set matches the token key id '{kid}'")]
    UnknownKeyId { kid: String },
}

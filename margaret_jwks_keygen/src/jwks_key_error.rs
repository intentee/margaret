use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwksKeyError {
    #[error("the claims to sign could not be serialized to json: {source}")]
    ClaimsSerialization {
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

    #[error("the jwk public key coordinates could not be built into a verifying key: {source}")]
    MalformedVerifyingKey {
        #[source]
        source: p256::ecdsa::Error,
    },

    #[error("the generated public key is missing its {coordinate} coordinate")]
    MissingPublicKeyCoordinate { coordinate: &'static str },

    #[error("the generated private key could not be encoded to pkcs#8 pem: {source}")]
    PemEncoding {
        #[from]
        source: p256::pkcs8::Error,
    },

    #[error("the jwk rsa public exponent is not valid base64url: {source}")]
    RsaExponentBase64 {
        #[source]
        source: base64ct::Error,
    },

    #[error("the jwk rsa public modulus is not valid base64url: {source}")]
    RsaModulusBase64 {
        #[source]
        source: base64ct::Error,
    },

    #[error(
        "the jwk rsa public modulus decodes to {found} bytes but rs256 verification requires at least {expected}"
    )]
    RsaModulusTooShort { expected: usize, found: usize },

    #[error("the signing key pem could not be parsed into an ecdsa signing key: {source}")]
    SigningKeyRejected {
        #[source]
        source: p256::pkcs8::Error,
    },
}

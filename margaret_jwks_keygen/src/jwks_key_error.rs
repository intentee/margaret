use aws_lc_rs::error::KeyRejected;
use aws_lc_rs::error::Unspecified;
use p256::pkcs8;
use thiserror::Error;

use margaret_jws_verification::duplicate_key_id::DuplicateKeyId;

#[derive(Debug, Error)]
pub enum JwksKeyError {
    #[error("the secret's keys do not form a key set: {duplicate}")]
    DuplicateKeyId { duplicate: DuplicateKeyId },

    #[error("the public key is missing its {coordinate} coordinate")]
    MissingPublicKeyCoordinate { coordinate: &'static str },

    #[error("the generated private key could not be encoded to pkcs#8 pem: {source}")]
    PemEncoding {
        #[source]
        source: pkcs8::Error,
    },

    #[error("the persisted rsa signing key is not base64: {source}")]
    RsaKeyBase64 {
        #[source]
        source: base64ct::Error,
    },

    #[error("the rsa signing key could not be encoded to pkcs#8: {source}")]
    RsaKeyEncoding {
        #[source]
        source: Unspecified,
    },

    #[error("the rsa signing key could not be generated: {source}")]
    RsaKeyGeneration {
        #[source]
        source: Unspecified,
    },

    #[error("the pkcs#8 document is not an rsa signing key: {source}")]
    RsaKeyRejected {
        #[source]
        source: KeyRejected,
    },

    #[error("the rsa signing key could not sign: {source}")]
    RsaSigning {
        #[source]
        source: Unspecified,
    },

    #[error("the signing key pem could not be parsed into an ecdsa signing key: {source}")]
    SigningKeyRejected {
        #[source]
        source: pkcs8::Error,
    },

    #[error("the public key of the signing key does not verify signatures: {source}")]
    VerificationKeyRejected {
        #[source]
        source: KeyRejected,
    },
}

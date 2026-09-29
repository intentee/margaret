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

    #[error("the signing key pem could not be parsed into an ecdsa signing key: {source}")]
    SigningKeyRejected {
        #[source]
        source: pkcs8::Error,
    },
}

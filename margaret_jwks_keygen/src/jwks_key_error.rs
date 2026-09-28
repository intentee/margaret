use p256::pkcs8;
use thiserror::Error;

use margaret_jws_verification::key_set_rejection::KeySetRejection;

#[derive(Debug, Error)]
pub enum JwksKeyError {
    #[error("the claims to sign could not be serialized to json: {source}")]
    ClaimsSerialization {
        #[source]
        source: serde_json::Error,
    },

    #[error("the secret's keys do not form a valid key set: {rejection}")]
    KeySetRejected { rejection: KeySetRejection },

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

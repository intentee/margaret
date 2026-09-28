use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use aws_lc_rs::error::Unspecified;
use p256::ecdsa;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::key_id::KeyId;

#[derive(Debug)]
pub enum JwsRejection {
    AlgorithmMismatch {
        key: JwsAlgorithm,
        token: JwsAlgorithm,
    },
    CriticalHeader,
    EcdsaSignatureMalformed {
        source: ecdsa::Error,
    },
    EcdsaSignatureMismatch {
        source: ecdsa::Error,
    },
    HeaderBase64 {
        source: base64ct::Error,
    },
    HeaderMalformed {
        source: serde_json::Error,
    },
    MissingKeyId,
    NotCompactJws,
    PayloadBase64 {
        source: base64ct::Error,
    },
    RsaSignatureMismatch {
        source: Unspecified,
    },
    SignatureBase64 {
        source: base64ct::Error,
    },
    UnknownKeyId {
        kid: KeyId,
    },
    UnsupportedAlgorithm {
        alg: String,
    },
}

impl Display for JwsRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::AlgorithmMismatch { key, token } => write!(
                formatter,
                "the token is signed with {token} but its key verifies {key}"
            ),
            Self::CriticalHeader => write!(
                formatter,
                "the token requires header extensions that are not understood"
            ),
            Self::EcdsaSignatureMalformed { source } => write!(
                formatter,
                "the token signature is not a valid ecdsa signature encoding: {source}"
            ),
            Self::EcdsaSignatureMismatch { source } => write!(
                formatter,
                "the token ecdsa signature does not match its key: {source}"
            ),
            Self::HeaderBase64 { source } => write!(
                formatter,
                "the token header segment is not valid base64url: {source}"
            ),
            Self::HeaderMalformed { source } => {
                write!(formatter, "the token header is not a jws header: {source}")
            }
            Self::MissingKeyId => write!(formatter, "the token header names no key id"),
            Self::NotCompactJws => {
                write!(
                    formatter,
                    "the token is not a compact jws of three segments"
                )
            }
            Self::PayloadBase64 { source } => write!(
                formatter,
                "the token payload segment is not valid base64url: {source}"
            ),
            Self::RsaSignatureMismatch { source } => write!(
                formatter,
                "the token rsa signature does not match its key: {source}"
            ),
            Self::SignatureBase64 { source } => write!(
                formatter,
                "the token signature segment is not valid base64url: {source}"
            ),
            Self::UnknownKeyId { kid } => {
                write!(formatter, "no key in the set carries the key id '{kid}'")
            }
            Self::UnsupportedAlgorithm { alg } => {
                write!(formatter, "the token algorithm '{alg}' is not supported")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aws_lc_rs::error::Unspecified;
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;
    use p256::ecdsa::Signature;

    use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

    use super::JwsRejection;
    use crate::key_id::KeyId;

    fn base64_error() -> base64ct::Error {
        Base64UrlUnpadded::decode_vec("!!!").expect_err("the fixture is not base64url")
    }

    fn signature_error() -> p256::ecdsa::Error {
        Signature::from_slice(&[0_u8; 3]).expect_err("the fixture is not a signature")
    }

    #[test]
    fn describes_every_rejection() {
        let described = [
            JwsRejection::AlgorithmMismatch {
                key: JwsAlgorithm::Es256,
                token: JwsAlgorithm::Es384,
            },
            JwsRejection::CriticalHeader,
            JwsRejection::HeaderBase64 {
                source: base64_error(),
            },
            JwsRejection::HeaderMalformed {
                source: serde_json::from_str::<u8>("x").expect_err("the fixture is not json"),
            },
            JwsRejection::MissingKeyId,
            JwsRejection::NotCompactJws,
            JwsRejection::PayloadBase64 {
                source: base64_error(),
            },
            JwsRejection::SignatureBase64 {
                source: base64_error(),
            },
            JwsRejection::EcdsaSignatureMalformed {
                source: signature_error(),
            },
            JwsRejection::EcdsaSignatureMismatch {
                source: signature_error(),
            },
            JwsRejection::RsaSignatureMismatch {
                source: Unspecified,
            },
            JwsRejection::UnknownKeyId {
                kid: KeyId::new("absent".to_string()),
            },
            JwsRejection::UnsupportedAlgorithm {
                alg: "none".to_string(),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "the token is signed with ES384 but its key verifies ES256"
        );
        assert!(described[1].contains("header extensions"));
        assert!(described[2].contains("header segment is not valid base64url"));
        assert!(described[3].contains("header is not a jws header"));
        assert_eq!(described[4], "the token header names no key id");
        assert_eq!(
            described[5],
            "the token is not a compact jws of three segments"
        );
        assert!(described[6].contains("payload segment is not valid base64url"));
        assert!(described[7].contains("signature segment is not valid base64url"));
        assert!(described[8].contains("not a valid ecdsa signature encoding"));
        assert!(described[9].contains("ecdsa signature does not match its key"));
        assert!(described[10].contains("rsa signature does not match its key"));
        assert!(described[11].contains("'absent'"));
        assert!(described[12].contains("'none'"));
    }
}

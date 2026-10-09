use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use aws_lc_rs::error::Unspecified;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::key_id::KeyId;
use crate::key_selection::KeySelection;

#[derive(Debug)]
pub enum JwsRejection {
    AlgorithmMismatch {
        key: JwsAlgorithm,
        token: JwsAlgorithm,
    },
    CriticalHeader,
    HeaderBase64 {
        source: base64ct::Error,
    },
    HeaderMalformed {
        source: serde_json::Error,
    },
    KeyIdRequired {
        candidates: usize,
    },
    NoKeyForAlgorithm {
        algorithm: JwsAlgorithm,
    },
    NotCompactJws,
    PayloadBase64 {
        source: base64ct::Error,
    },
    SignatureBase64 {
        source: base64ct::Error,
    },
    SignatureLength {
        expected: usize,
        found: usize,
    },
    SignatureMismatch {
        algorithm: JwsAlgorithm,
        selection: KeySelection,
        source: Unspecified,
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
            Self::HeaderBase64 { source } => write!(
                formatter,
                "the token header segment is not valid base64url: {source}"
            ),
            Self::HeaderMalformed { source } => {
                write!(formatter, "the token header is not a jws header: {source}")
            }
            Self::KeyIdRequired { candidates } => write!(
                formatter,
                "the token header names no key id, and {candidates} keys of the set verify its algorithm instead of exactly one"
            ),
            Self::NoKeyForAlgorithm { algorithm } => write!(
                formatter,
                "the token header names no key id, and no key of the set verifies {algorithm}"
            ),
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
            Self::SignatureBase64 { source } => write!(
                formatter,
                "the token signature segment is not valid base64url: {source}"
            ),
            Self::SignatureLength { expected, found } => write!(
                formatter,
                "the token signature has {found} octets where its key produces {expected}"
            ),
            Self::SignatureMismatch {
                algorithm, source, ..
            } => write!(
                formatter,
                "the token {algorithm} signature does not match its key: {source}"
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

    use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

    use super::JwsRejection;
    use crate::key_id::KeyId;
    use crate::key_selection::KeySelection;

    fn base64_error() -> base64ct::Error {
        Base64UrlUnpadded::decode_vec("!!!").expect_err("the fixture is not base64url")
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
            JwsRejection::KeyIdRequired { candidates: 2 },
            JwsRejection::NoKeyForAlgorithm {
                algorithm: JwsAlgorithm::Es512,
            },
            JwsRejection::NotCompactJws,
            JwsRejection::PayloadBase64 {
                source: base64_error(),
            },
            JwsRejection::SignatureBase64 {
                source: base64_error(),
            },
            JwsRejection::SignatureLength {
                expected: 64,
                found: 3,
            },
            JwsRejection::SignatureMismatch {
                algorithm: JwsAlgorithm::Ps256,
                selection: KeySelection::ByKeyId,
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
        assert_eq!(
            described[4],
            "the token header names no key id, and 2 keys of the set verify its algorithm instead of exactly one"
        );
        assert_eq!(
            described[5],
            "the token header names no key id, and no key of the set verifies ES512"
        );
        assert_eq!(
            described[6],
            "the token is not a compact jws of three segments"
        );
        assert!(described[7].contains("payload segment is not valid base64url"));
        assert!(described[8].contains("signature segment is not valid base64url"));
        assert_eq!(
            described[9],
            "the token signature has 3 octets where its key produces 64"
        );
        assert!(described[10].starts_with("the token PS256 signature does not match its key: "));
        assert!(described[11].contains("'absent'"));
        assert!(described[12].contains("'none'"));
    }
}

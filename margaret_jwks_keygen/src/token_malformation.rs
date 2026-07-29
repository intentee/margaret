use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

use crate::jws_algorithm::JwsAlgorithm;

#[derive(Debug)]
pub enum TokenMalformation {
    AlgorithmMismatch {
        expected: JwsAlgorithm,
        found: JwsAlgorithm,
    },
    ClaimsBase64(base64ct::Error),
    ClaimsJson(serde_json::Error),
    HeaderBase64(base64ct::Error),
    HeaderJson(serde_json::Error),
    NonCanonicalSignature,
    NotCompactJws,
    SignatureBase64(base64ct::Error),
    SignatureMalformed(p256::ecdsa::Error),
    UnknownKeyId {
        kid: String,
    },
}

impl Display for TokenMalformation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        match self {
            Self::AlgorithmMismatch { expected, found } => write!(
                formatter,
                "the token algorithm {found:?} does not match the key algorithm {expected:?}"
            ),
            Self::ClaimsBase64(source) => {
                write!(
                    formatter,
                    "the token payload segment is not valid base64url: {source}"
                )
            }
            Self::ClaimsJson(source) => write!(
                formatter,
                "the token payload could not be handled as json: {source}"
            ),
            Self::HeaderBase64(source) => write!(
                formatter,
                "the token header segment is not valid base64url: {source}"
            ),
            Self::HeaderJson(source) => write!(
                formatter,
                "the token header could not be handled as json: {source}"
            ),
            Self::NonCanonicalSignature => {
                write!(
                    formatter,
                    "the token signature is not in canonical low-s form"
                )
            }
            Self::NotCompactJws => {
                write!(formatter, "the token is not a well-formed compact jws")
            }
            Self::SignatureBase64(source) => write!(
                formatter,
                "the token signature segment is not valid base64url: {source}"
            ),
            Self::SignatureMalformed(source) => write!(
                formatter,
                "the token signature is not a valid ecdsa signature: {source}"
            ),
            Self::UnknownKeyId { kid } => {
                write!(
                    formatter,
                    "no jwk in the set matches the token key id '{kid}'"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding as _;

    use super::TokenMalformation;
    use crate::jws_algorithm::JwsAlgorithm;

    fn base64_error() -> base64ct::Error {
        Base64UrlUnpadded::decode_vec("!!!").expect_err("the fixture is not base64url")
    }

    fn json_error() -> serde_json::Error {
        serde_json::from_str::<serde_json::Value>("not json").expect_err("the fixture is not json")
    }

    fn signature_error() -> p256::ecdsa::Error {
        p256::ecdsa::Signature::from_slice(&[0u8; 3])
            .expect_err("the fixture is not a valid signature")
    }

    #[test]
    fn describes_every_way_a_token_can_be_malformed() {
        let described = [
            TokenMalformation::AlgorithmMismatch {
                expected: JwsAlgorithm::Es256,
                found: JwsAlgorithm::Es384,
            },
            TokenMalformation::ClaimsBase64(base64_error()),
            TokenMalformation::ClaimsJson(json_error()),
            TokenMalformation::HeaderBase64(base64_error()),
            TokenMalformation::HeaderJson(json_error()),
            TokenMalformation::NonCanonicalSignature,
            TokenMalformation::NotCompactJws,
            TokenMalformation::SignatureBase64(base64_error()),
            TokenMalformation::SignatureMalformed(signature_error()),
            TokenMalformation::UnknownKeyId {
                kid: "absent".to_string(),
            },
        ]
        .map(|malformation| malformation.to_string());

        assert!(described[0].contains("does not match the key algorithm"));
        assert!(described[1].contains("payload segment is not valid base64url"));
        assert!(described[2].contains("payload could not be handled as json"));
        assert!(described[3].contains("header segment is not valid base64url"));
        assert!(described[4].contains("header could not be handled as json"));
        assert_eq!(
            described[5],
            "the token signature is not in canonical low-s form"
        );
        assert_eq!(described[6], "the token is not a well-formed compact jws");
        assert!(described[7].contains("signature segment is not valid base64url"));
        assert!(described[8].contains("not a valid ecdsa signature"));
        assert!(described[9].contains("'absent'"));
    }
}

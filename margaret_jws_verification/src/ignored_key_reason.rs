use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::jwk_rejection::JwkRejection;

#[derive(Debug)]
pub enum IgnoredKeyReason {
    Malformed { source: serde_json::Error },
    UnrecognizedUse { key_use: String },
    UnsupportedAlgorithm { alg: String },
    UnsupportedCurve { crv: String },
    UnsupportedKeyType { kty: String },
    Unusable { rejection: JwkRejection },
}

impl Display for IgnoredKeyReason {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Malformed { source } => {
                write!(formatter, "the key is not a well-formed jwk: {source}")
            }
            Self::UnrecognizedUse { key_use } => {
                write!(
                    formatter,
                    "the key declares the unrecognized use '{key_use}'"
                )
            }
            Self::UnsupportedAlgorithm { alg } => {
                write!(
                    formatter,
                    "the key declares the unsupported algorithm '{alg}'"
                )
            }
            Self::UnsupportedCurve { crv } => {
                write!(formatter, "the key is on the unsupported curve '{crv}'")
            }
            Self::UnsupportedKeyType { kty } => {
                write!(formatter, "the key is of the unsupported type '{kty}'")
            }
            Self::Unusable { rejection } => rejection.fmt(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IgnoredKeyReason;
    use crate::jwk_rejection::JwkRejection;

    #[test]
    fn describes_every_reason() {
        let described = [
            IgnoredKeyReason::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
            IgnoredKeyReason::UnrecognizedUse {
                key_use: "tls".to_string(),
            },
            IgnoredKeyReason::UnsupportedAlgorithm {
                alg: "RSA-OAEP".to_string(),
            },
            IgnoredKeyReason::UnsupportedCurve {
                crv: "P-521".to_string(),
            },
            IgnoredKeyReason::UnsupportedKeyType {
                kty: "oct".to_string(),
            },
            IgnoredKeyReason::Unusable {
                rejection: JwkRejection::MissingKeyId,
            },
        ]
        .map(|reason| reason.to_string());

        assert!(described[0].starts_with("the key is not a well-formed jwk: "));
        assert_eq!(described[1], "the key declares the unrecognized use 'tls'");
        assert_eq!(
            described[2],
            "the key declares the unsupported algorithm 'RSA-OAEP'"
        );
        assert_eq!(described[3], "the key is on the unsupported curve 'P-521'");
        assert_eq!(described[4], "the key is of the unsupported type 'oct'");
        assert_eq!(described[5], "the key has no key id");
    }
}

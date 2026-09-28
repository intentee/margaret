use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use p256::ecdsa;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

#[derive(Debug)]
pub enum JwkRejection {
    AlgorithmMismatch {
        algorithm: JwsAlgorithm,
        curve: Curve,
    },
    CoordinateBase64 {
        source: base64ct::Error,
    },
    CoordinateLength {
        expected: usize,
        found: usize,
    },
    InvalidPoint {
        source: ecdsa::Error,
    },
    Malformed {
        source: serde_json::Error,
    },
    MissingKeyId,
    UnsupportedUse,
}

impl Display for JwkRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::AlgorithmMismatch { algorithm, curve } => write!(
                formatter,
                "the key declares the algorithm {algorithm}, which the curve {curve:?} cannot produce"
            ),
            Self::CoordinateBase64 { source } => write!(
                formatter,
                "the key coordinate is not valid base64url: {source}"
            ),
            Self::CoordinateLength { expected, found } => write!(
                formatter,
                "the key coordinate decodes to {found} bytes but the curve requires exactly {expected}"
            ),
            Self::InvalidPoint { source } => write!(
                formatter,
                "the key coordinates are not a point on the curve: {source}"
            ),
            Self::Malformed { source } => {
                write!(formatter, "the key is not a supported jwk: {source}")
            }
            Self::MissingKeyId => write!(formatter, "the key has no key id"),
            Self::UnsupportedUse => write!(formatter, "the key is not meant for signatures"),
        }
    }
}

#[cfg(test)]
mod tests {
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;
    use p256::ecdsa::VerifyingKey;

    use margaret_jose_parameters::curve::Curve;
    use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

    use super::JwkRejection;

    #[test]
    fn describes_every_rejection() {
        let described = [
            JwkRejection::AlgorithmMismatch {
                algorithm: JwsAlgorithm::Es384,
                curve: Curve::P256,
            },
            JwkRejection::CoordinateBase64 {
                source: Base64UrlUnpadded::decode_vec("!!!").expect_err("not base64url"),
            },
            JwkRejection::CoordinateLength {
                expected: 32,
                found: 31,
            },
            JwkRejection::InvalidPoint {
                source: VerifyingKey::from_sec1_bytes(&[4]).expect_err("not a point"),
            },
            JwkRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
            JwkRejection::MissingKeyId,
            JwkRejection::UnsupportedUse,
        ]
        .map(|rejection| rejection.to_string());

        assert!(described[0].contains("declares the algorithm ES384"));
        assert!(described[1].contains("not valid base64url"));
        assert!(described[2].contains("decodes to 31 bytes but the curve requires exactly 32"));
        assert!(described[3].contains("not a point on the curve"));
        assert!(described[4].contains("not a supported jwk"));
        assert_eq!(described[5], "the key has no key id");
        assert_eq!(described[6], "the key is not meant for signatures");
    }
}

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use aws_lc_rs::error::KeyRejected;
use aws_lc_rs::error::Unspecified;
use p256::ecdsa;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

#[derive(Debug)]
pub enum JwkRejection {
    AlgorithmMismatch {
        declared: JwsAlgorithm,
        implied: JwsAlgorithm,
    },
    CoordinateBase64 {
        source: base64ct::Error,
    },
    CoordinateLength {
        expected: usize,
        found: usize,
    },
    ExponentBase64 {
        source: base64ct::Error,
    },
    InvalidPoint {
        source: ecdsa::Error,
    },
    InvalidRsaKey {
        source: KeyRejected,
    },
    Malformed {
        source: serde_json::Error,
    },
    MissingKeyId,
    ModulusBase64 {
        source: base64ct::Error,
    },
    ModulusSize {
        bits: u64,
    },
    RsaComponentsNotMinimal {
        source: Unspecified,
    },
    UnsupportedUse,
}

impl Display for JwkRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::AlgorithmMismatch { declared, implied } => write!(
                formatter,
                "the key declares the algorithm {declared}, but its key material verifies {implied}"
            ),
            Self::CoordinateBase64 { source } => write!(
                formatter,
                "the key coordinate is not valid base64url: {source}"
            ),
            Self::CoordinateLength { expected, found } => write!(
                formatter,
                "the key coordinate decodes to {found} bytes but the curve requires exactly {expected}"
            ),
            Self::ExponentBase64 { source } => write!(
                formatter,
                "the key exponent is not valid base64url: {source}"
            ),
            Self::InvalidPoint { source } => write!(
                formatter,
                "the key coordinates are not a point on the curve: {source}"
            ),
            Self::InvalidRsaKey { source } => {
                write!(formatter, "the key is not a valid rsa public key: {source}")
            }
            Self::Malformed { source } => {
                write!(formatter, "the key is not a supported jwk: {source}")
            }
            Self::MissingKeyId => write!(formatter, "the key has no key id"),
            Self::ModulusBase64 { source } => {
                write!(
                    formatter,
                    "the key modulus is not valid base64url: {source}"
                )
            }
            Self::ModulusSize { bits } => write!(
                formatter,
                "the key modulus has {bits} bits, outside the supported range"
            ),
            Self::RsaComponentsNotMinimal { source } => write!(
                formatter,
                "the key modulus or exponent is not a minimal unsigned integer: {source}"
            ),
            Self::UnsupportedUse => write!(formatter, "the key is not meant for signatures"),
        }
    }
}

#[cfg(test)]
mod tests {
    use aws_lc_rs::error::Unspecified;
    use aws_lc_rs::signature::ParsedPublicKey;
    use aws_lc_rs::signature::RSA_PKCS1_2048_8192_SHA256;
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;
    use p256::ecdsa::VerifyingKey;

    use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

    use super::JwkRejection;

    fn base64_error() -> base64ct::Error {
        Base64UrlUnpadded::decode_vec("!!!").expect_err("not base64url")
    }

    #[test]
    fn describes_every_rejection() {
        let described = [
            JwkRejection::AlgorithmMismatch {
                declared: JwsAlgorithm::Es384,
                implied: JwsAlgorithm::Es256,
            },
            JwkRejection::CoordinateBase64 {
                source: base64_error(),
            },
            JwkRejection::CoordinateLength {
                expected: 32,
                found: 31,
            },
            JwkRejection::ExponentBase64 {
                source: base64_error(),
            },
            JwkRejection::InvalidPoint {
                source: VerifyingKey::from_sec1_bytes(&[4]).expect_err("not a point"),
            },
            JwkRejection::InvalidRsaKey {
                source: ParsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, b"not der")
                    .expect_err("not an rsa key"),
            },
            JwkRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
            JwkRejection::MissingKeyId,
            JwkRejection::ModulusBase64 {
                source: base64_error(),
            },
            JwkRejection::ModulusSize { bits: 1024 },
            JwkRejection::RsaComponentsNotMinimal {
                source: Unspecified,
            },
            JwkRejection::UnsupportedUse,
        ]
        .map(|rejection| rejection.to_string());

        assert!(described[0].contains("declares the algorithm ES384"));
        assert!(described[0].contains("verifies ES256"));
        assert!(described[1].contains("coordinate is not valid base64url"));
        assert!(described[2].contains("decodes to 31 bytes but the curve requires exactly 32"));
        assert!(described[3].contains("exponent is not valid base64url"));
        assert!(described[4].contains("not a point on the curve"));
        assert!(described[5].contains("not a valid rsa public key"));
        assert!(described[6].contains("not a supported jwk"));
        assert_eq!(described[7], "the key has no key id");
        assert!(described[8].contains("modulus is not valid base64url"));
        assert!(described[9].contains("1024 bits"));
        assert!(described[10].contains("not a minimal unsigned integer"));
        assert_eq!(described[11], "the key is not meant for signatures");
    }
}

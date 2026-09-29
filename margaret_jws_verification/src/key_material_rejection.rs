use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use aws_lc_rs::error::KeyRejected;
use aws_lc_rs::error::Unspecified;
use p256::ecdsa;

#[derive(Debug)]
pub enum KeyMaterialRejection {
    CoordinateBase64 { source: base64ct::Error },
    CoordinateLength { expected: usize, found: usize },
    ExponentBase64 { source: base64ct::Error },
    InvalidPoint { source: ecdsa::Error },
    InvalidRsaKey { source: KeyRejected },
    MalformedMembers { source: serde_json::Error },
    ModulusBase64 { source: base64ct::Error },
    ModulusSize { bits: u64 },
    RsaComponentsNotMinimal { source: Unspecified },
    UnsupportedCurve { crv: String },
}

impl Display for KeyMaterialRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
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
            Self::MalformedMembers { source } => write!(
                formatter,
                "the key's type-specific members are malformed: {source}"
            ),
            Self::ModulusBase64 { source } => write!(
                formatter,
                "the key modulus is not valid base64url: {source}"
            ),
            Self::ModulusSize { bits } => write!(
                formatter,
                "the key modulus has {bits} bits, outside the supported range"
            ),
            Self::RsaComponentsNotMinimal { source } => write!(
                formatter,
                "the key modulus or exponent is not a minimal unsigned integer: {source}"
            ),
            Self::UnsupportedCurve { crv } => {
                write!(formatter, "the key is on the unsupported curve '{crv}'")
            }
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

    use super::KeyMaterialRejection;

    fn base64_error() -> base64ct::Error {
        Base64UrlUnpadded::decode_vec("!!!").expect_err("not base64url")
    }

    #[test]
    fn describes_every_rejection() {
        let described = [
            KeyMaterialRejection::CoordinateBase64 {
                source: base64_error(),
            },
            KeyMaterialRejection::CoordinateLength {
                expected: 32,
                found: 31,
            },
            KeyMaterialRejection::ExponentBase64 {
                source: base64_error(),
            },
            KeyMaterialRejection::InvalidPoint {
                source: VerifyingKey::from_sec1_bytes(&[4]).expect_err("not a point"),
            },
            KeyMaterialRejection::InvalidRsaKey {
                source: ParsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, b"not der")
                    .expect_err("not an rsa key"),
            },
            KeyMaterialRejection::MalformedMembers {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
            KeyMaterialRejection::ModulusBase64 {
                source: base64_error(),
            },
            KeyMaterialRejection::ModulusSize { bits: 1024 },
            KeyMaterialRejection::RsaComponentsNotMinimal {
                source: Unspecified,
            },
            KeyMaterialRejection::UnsupportedCurve {
                crv: "P-521".to_string(),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert!(described[0].contains("coordinate is not valid base64url"));
        assert!(described[1].contains("decodes to 31 bytes but the curve requires exactly 32"));
        assert!(described[2].contains("exponent is not valid base64url"));
        assert!(described[3].contains("not a point on the curve"));
        assert!(described[4].contains("not a valid rsa public key"));
        assert!(described[5].starts_with("the key's type-specific members are malformed: "));
        assert!(described[6].contains("modulus is not valid base64url"));
        assert!(described[7].contains("1024 bits"));
        assert!(described[8].contains("not a minimal unsigned integer"));
        assert_eq!(described[9], "the key is on the unsupported curve 'P-521'");
    }
}

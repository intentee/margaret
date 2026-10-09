use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::certificate_rejection::CertificateRejection;
use crate::key_algorithm_rejection::KeyAlgorithmRejection;
use crate::key_material_rejection::KeyMaterialRejection;
use crate::key_usage_rejection::KeyUsageRejection;

#[derive(Debug)]
pub enum IgnoredKeyReason {
    Algorithm(KeyAlgorithmRejection),
    Certificate(CertificateRejection),
    Malformed { source: serde_json::Error },
    Material(KeyMaterialRejection),
    UnsupportedKeyType { kty: String },
    Usage(KeyUsageRejection),
}

impl Display for IgnoredKeyReason {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Algorithm(rejection) => rejection.fmt(formatter),
            Self::Certificate(rejection) => rejection.fmt(formatter),
            Self::Malformed { source } => {
                write!(formatter, "the key is not a well-formed jwk: {source}")
            }
            Self::Material(rejection) => rejection.fmt(formatter),
            Self::UnsupportedKeyType { kty } => {
                write!(formatter, "the key is of the unsupported type '{kty}'")
            }
            Self::Usage(rejection) => rejection.fmt(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IgnoredKeyReason;
    use crate::certificate_rejection::CertificateRejection;
    use crate::key_algorithm_rejection::KeyAlgorithmRejection;
    use crate::key_material_rejection::KeyMaterialRejection;
    use crate::key_usage_rejection::KeyUsageRejection;

    #[test]
    fn describes_every_reason() {
        let described = [
            IgnoredKeyReason::Algorithm(KeyAlgorithmRejection::UnsupportedAlgorithm {
                alg: "PS256".to_string(),
            }),
            IgnoredKeyReason::Certificate(CertificateRejection::EmptyChain),
            IgnoredKeyReason::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
            IgnoredKeyReason::Material(KeyMaterialRejection::ModulusSize { bits: 1024 }),
            IgnoredKeyReason::UnsupportedKeyType {
                kty: "OKP".to_string(),
            },
            IgnoredKeyReason::Usage(KeyUsageRejection::EncryptionUse),
        ]
        .map(|reason| reason.to_string());

        assert_eq!(
            described[0],
            "the key declares the unsupported algorithm 'PS256'"
        );
        assert_eq!(described[1], "the key's certificate chain is empty");
        assert!(described[2].starts_with("the key is not a well-formed jwk: "));
        assert!(described[3].contains("1024 bits"));
        assert_eq!(described[4], "the key is of the unsupported type 'OKP'");
        assert_eq!(described[5], "the key is meant for encryption");
    }
}

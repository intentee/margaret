use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use aws_lc_rs::error::KeyRejected;
use aws_lc_rs::error::Unspecified;

#[derive(Debug)]
pub enum CertificateRejection {
    CertificateBase64 { source: base64ct::Error },
    CertificateKeyUnencodable(Unspecified),
    CertificateKeyUnreadable(KeyRejected),
    CertificateMalformed { source: webpki::Error },
    EmptyChain,
    KeyMismatch,
    PublishedKeyUnencodable(Unspecified),
    Sha1ThumbprintBase64 { source: base64ct::Error },
    Sha1ThumbprintMismatch,
    Sha256ThumbprintBase64 { source: base64ct::Error },
    Sha256ThumbprintMismatch,
}

impl Display for CertificateRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::CertificateBase64 { source } => write!(
                formatter,
                "the key's first certificate is not valid base64: {source}"
            ),
            Self::CertificateKeyUnencodable(source) => write!(
                formatter,
                "the public key of the key's first certificate cannot be encoded for comparison: {source}"
            ),
            Self::CertificateKeyUnreadable(source) => write!(
                formatter,
                "the key's first certificate holds no key of the same algorithm: {source}"
            ),
            Self::CertificateMalformed { source } => write!(
                formatter,
                "the key's first certificate is not an x.509 certificate: {source}"
            ),
            Self::EmptyChain => write!(formatter, "the key's certificate chain is empty"),
            Self::KeyMismatch => write!(
                formatter,
                "the key's first certificate holds a different public key"
            ),
            Self::PublishedKeyUnencodable(source) => write!(
                formatter,
                "the published key cannot be encoded for comparison with its certificate: {source}"
            ),
            Self::Sha1ThumbprintBase64 { source } => write!(
                formatter,
                "the key's x5t thumbprint is not valid base64url: {source}"
            ),
            Self::Sha1ThumbprintMismatch => write!(
                formatter,
                "the key's x5t thumbprint does not match its first certificate"
            ),
            Self::Sha256ThumbprintBase64 { source } => write!(
                formatter,
                "the key's x5t#S256 thumbprint is not valid base64url: {source}"
            ),
            Self::Sha256ThumbprintMismatch => write!(
                formatter,
                "the key's x5t#S256 thumbprint does not match its first certificate"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use aws_lc_rs::error::Unspecified;
    use aws_lc_rs::signature::ECDSA_P256_SHA256_FIXED;
    use aws_lc_rs::signature::ParsedPublicKey;
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;
    use rustls_pki_types::CertificateDer;
    use webpki::EndEntityCert;

    use super::CertificateRejection;

    fn base64_error() -> base64ct::Error {
        Base64UrlUnpadded::decode_vec("!!!").expect_err("not base64url")
    }

    #[test]
    fn describes_every_rejection() {
        let not_a_certificate = CertificateDer::from(vec![0_u8]);
        let described = [
            CertificateRejection::CertificateBase64 {
                source: base64_error(),
            },
            CertificateRejection::CertificateKeyUnencodable(Unspecified),
            CertificateRejection::CertificateKeyUnreadable(
                ParsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, [0_u8]).expect_err("not a key"),
            ),
            CertificateRejection::CertificateMalformed {
                source: EndEntityCert::try_from(&not_a_certificate)
                    .err()
                    .expect("not a certificate"),
            },
            CertificateRejection::EmptyChain,
            CertificateRejection::KeyMismatch,
            CertificateRejection::PublishedKeyUnencodable(Unspecified),
            CertificateRejection::Sha1ThumbprintBase64 {
                source: base64_error(),
            },
            CertificateRejection::Sha1ThumbprintMismatch,
            CertificateRejection::Sha256ThumbprintBase64 {
                source: base64_error(),
            },
            CertificateRejection::Sha256ThumbprintMismatch,
        ]
        .map(|rejection| rejection.to_string());

        assert!(described[0].starts_with("the key's first certificate is not valid base64: "));
        assert!(described[1].starts_with(
            "the public key of the key's first certificate cannot be encoded for comparison: "
        ));
        assert!(
            described[2]
                .starts_with("the key's first certificate holds no key of the same algorithm: ")
        );
        assert!(
            described[3].starts_with("the key's first certificate is not an x.509 certificate: ")
        );
        assert_eq!(described[4], "the key's certificate chain is empty");
        assert_eq!(
            described[5],
            "the key's first certificate holds a different public key"
        );
        assert!(described[6].starts_with(
            "the published key cannot be encoded for comparison with its certificate: "
        ));
        assert!(described[7].starts_with("the key's x5t thumbprint is not valid base64url: "));
        assert_eq!(
            described[8],
            "the key's x5t thumbprint does not match its first certificate"
        );
        assert!(described[9].starts_with("the key's x5t#S256 thumbprint is not valid base64url: "));
        assert_eq!(
            described[10],
            "the key's x5t#S256 thumbprint does not match its first certificate"
        );
    }
}

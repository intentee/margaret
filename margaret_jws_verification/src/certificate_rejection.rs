use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use p256::pkcs8::spki;

#[derive(Debug)]
pub enum CertificateRejection {
    CertificateBase64 { source: base64ct::Error },
    CertificateKeyUnreadable { source: spki::Error },
    CertificateMalformed { source: webpki::Error },
    EmptyChain,
    KeyMismatch,
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
            Self::CertificateKeyUnreadable { source } => write!(
                formatter,
                "the key's first certificate holds no key of the same curve: {source}"
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
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;
    use p256::ecdsa::VerifyingKey;
    use p256::pkcs8::DecodePublicKey;
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
            CertificateRejection::CertificateKeyUnreadable {
                source: VerifyingKey::from_public_key_der(&[0_u8]).expect_err("not a key"),
            },
            CertificateRejection::CertificateMalformed {
                source: EndEntityCert::try_from(&not_a_certificate)
                    .err()
                    .expect("not a certificate"),
            },
            CertificateRejection::EmptyChain,
            CertificateRejection::KeyMismatch,
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
        assert!(
            described[1]
                .starts_with("the key's first certificate holds no key of the same curve: ")
        );
        assert!(
            described[2].starts_with("the key's first certificate is not an x.509 certificate: ")
        );
        assert_eq!(described[3], "the key's certificate chain is empty");
        assert_eq!(
            described[4],
            "the key's first certificate holds a different public key"
        );
        assert!(described[5].starts_with("the key's x5t thumbprint is not valid base64url: "));
        assert_eq!(
            described[6],
            "the key's x5t thumbprint does not match its first certificate"
        );
        assert!(described[7].starts_with("the key's x5t#S256 thumbprint is not valid base64url: "));
        assert_eq!(
            described[8],
            "the key's x5t#S256 thumbprint does not match its first certificate"
        );
    }
}

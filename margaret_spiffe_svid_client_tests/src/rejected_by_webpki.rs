use rustls::CertificateError;
use rustls::Error;
use rustls::OtherError;

#[must_use]
pub fn rejected_by_webpki(error: &Error, expected: &webpki::Error) -> bool {
    matches!(
        error,
        Error::InvalidCertificate(CertificateError::Other(OtherError(source)))
            if source.downcast_ref::<webpki::Error>() == Some(expected)
    )
}

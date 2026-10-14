use std::sync::Arc;

use rustls::CertificateError;
use rustls::Error;
use rustls::OtherError;
use rustls::pki_types::CertificateDer;
use webpki::EndEntityCert;

/// # Errors
///
/// Returns `Error::InvalidCertificate` carrying the webpki error that refused the certificate.
pub fn parse_end_entity_cert<'cert>(
    end_entity: &'cert CertificateDer<'cert>,
) -> Result<EndEntityCert<'cert>, Error> {
    EndEntityCert::try_from(end_entity).map_err(|source| {
        Error::InvalidCertificate(CertificateError::Other(OtherError(Arc::new(source))))
    })
}

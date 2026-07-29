use log::warn;
use rustls::CertificateError;
use rustls::Error;
use x509_parser::oid_registry::OID_X509_EXT_SUBJECT_ALT_NAME;
use x509_parser::prelude::FromDer as _;
use x509_parser::prelude::GeneralName;
use x509_parser::prelude::ParsedExtension;
use x509_parser::prelude::X509Certificate;

use crate::extract_spiffe_trust_domain_from_uri::extract_spiffe_trust_domain_from_uri;

/// # Errors
///
/// Returns `Error::InvalidCertificate`.
pub fn extract_spiffe_trust_domain(cert_der: &[u8]) -> Result<String, Error> {
    let (_, cert) = X509Certificate::from_der(cert_der).map_err(|err| {
        warn!("Unable to parse der certificate: {err:#?}");

        Error::InvalidCertificate(CertificateError::BadEncoding)
    })?;

    if let Ok(Some(san_ext)) = cert.get_extension_unique(&OID_X509_EXT_SUBJECT_ALT_NAME)
        && let ParsedExtension::SubjectAlternativeName(san) = san_ext.parsed_extension()
    {
        for name in &san.general_names {
            let GeneralName::URI(uri) = name else {
                continue;
            };

            if let Some(trust_domain) = extract_spiffe_trust_domain_from_uri(uri)? {
                return Ok(trust_domain);
            }
        }
    }

    Err(Error::InvalidCertificate(
        CertificateError::ApplicationVerificationFailure,
    ))
}

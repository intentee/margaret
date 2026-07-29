use std::str::FromStr as _;

use spiffe::spiffe_id::SpiffeId;
use x509_parser::prelude::FromDer as _;
use x509_parser::prelude::GeneralName;
use x509_parser::prelude::ParsedExtension;
use x509_parser::prelude::X509Certificate;

use crate::peer_identity_error::PeerIdentityError;

/// # Errors
///
/// Returns `PeerIdentityError::CertificateEncoding` or `PeerIdentityError::MissingUri` or `PeerIdentityError::SpiffeId`.
pub fn spiffe_id_from_cert(certificate_der: &[u8]) -> Result<SpiffeId, PeerIdentityError> {
    let (_remaining, certificate) =
        X509Certificate::from_der(certificate_der).map_err(|error| {
            PeerIdentityError::CertificateEncoding {
                source: error.into(),
            }
        })?;

    let uris: Vec<&str> = certificate
        .iter_extensions()
        .filter_map(|extension| match extension.parsed_extension() {
            ParsedExtension::SubjectAlternativeName(subject_alternative_name) => {
                Some(&subject_alternative_name.general_names)
            }
            _ => None,
        })
        .flatten()
        .filter_map(|general_name| match general_name {
            GeneralName::URI(uri) => Some(*uri),
            _ => None,
        })
        .collect();

    match uris.as_slice() {
        [] => Err(PeerIdentityError::MissingUri),
        [uri] => SpiffeId::from_str(uri).map_err(|source| PeerIdentityError::SpiffeId { source }),
        _ => Err(PeerIdentityError::MultipleUris { count: uris.len() }),
    }
}

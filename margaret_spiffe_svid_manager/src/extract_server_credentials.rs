use rustls::pki_types::CertificateDer;
use spiffe::X509Svid;

use crate::build_svid_certified_key::build_svid_certified_key;
use crate::svid_certified_key::SvidCertifiedKey;
use crate::svid_error::SvidError;

pub fn extract_server_credentials(
    default_svid: &X509Svid,
) -> Result<SvidCertifiedKey, SvidError> {
    let cert_chain: Vec<CertificateDer> = default_svid
        .cert_chain()
        .iter()
        .map(|cert| CertificateDer::from(cert.as_ref().to_vec()))
        .collect();

    build_svid_certified_key(cert_chain, default_svid.private_key().as_ref())
}

use std::sync::Arc;

use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::sign::CertifiedKey;

#[derive(Debug)]
pub struct SvidCertifiedKey {
    pub cert_chain: Vec<CertificateDer<'static>>,
    pub certified_key: Arc<CertifiedKey>,
    pub private_key_der: PrivateKeyDer<'static>,
}

use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;

pub struct IssuedCertificate {
    pub certificate_der: CertificateDer<'static>,
    pub private_key: PrivateKeyDer<'static>,
}

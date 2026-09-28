use rcgen::BasicConstraints;
use rcgen::CertificateParams;
use rcgen::DistinguishedName;
use rcgen::DnType;
use rcgen::IsCa;
use rcgen::Issuer;
use rcgen::KeyPair;
use rcgen::KeyUsagePurpose;
use rcgen::SanType;
use rustls::RootCertStore;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;

use crate::issued_certificate::IssuedCertificate;

fn named(common_name: &str) -> DistinguishedName {
    let mut distinguished_name = DistinguishedName::new();

    distinguished_name.push(DnType::CommonName, common_name);

    distinguished_name
}

pub struct FixtureCertificateAuthority {
    certificate_der: CertificateDer<'static>,
    issuer: Issuer<'static, KeyPair>,
}

impl FixtureCertificateAuthority {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    #[must_use]
    pub fn generate() -> Self {
        let key = KeyPair::generate().expect("the CA key pair generates");
        let mut params = CertificateParams::default();

        params.distinguished_name = named("margaret fixture certificate authority");
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.key_usages = vec![
            KeyUsagePurpose::KeyCertSign,
            KeyUsagePurpose::DigitalSignature,
        ];

        let certificate_der = params
            .self_signed(&key)
            .expect("the CA certificate self-signs")
            .der()
            .clone();

        Self {
            certificate_der,
            issuer: Issuer::new(params, key),
        }
    }

    #[must_use]
    pub fn certificate_der(&self) -> &CertificateDer<'static> {
        &self.certificate_der
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    #[must_use]
    pub fn issue(&self, common_name: &str, subject_alt_name: SanType) -> IssuedCertificate {
        let key = KeyPair::generate().expect("the leaf key pair generates");
        let mut params = CertificateParams::default();

        params.distinguished_name = named(common_name);
        params.subject_alt_names = vec![subject_alt_name];

        let certificate_der = params
            .signed_by(&key, &self.issuer)
            .expect("the leaf certificate is signed by the CA")
            .der()
            .clone();

        IssuedCertificate {
            certificate_der,
            private_key: PrivateKeyDer::Pkcs8(key.serialize_der().into()),
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    #[must_use]
    pub fn root_store(&self) -> RootCertStore {
        let mut roots = RootCertStore::empty();

        roots
            .add(self.certificate_der.clone())
            .expect("the CA certificate is added to the root store");

        roots
    }
}

use rcgen::BasicConstraints;
use rcgen::Certificate;
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
use crate::issued_pem_certificate::IssuedPemCertificate;

struct SignedLeaf {
    certificate: Certificate,
    key: KeyPair,
}

fn named(common_name: &str) -> DistinguishedName {
    let mut distinguished_name = DistinguishedName::new();

    distinguished_name.push(DnType::CommonName, common_name);

    distinguished_name
}

pub struct FixtureCertificateAuthority {
    certificate_der: CertificateDer<'static>,
    certificate_pem: String,
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

        let certificate = params
            .self_signed(&key)
            .expect("the CA certificate self-signs");

        Self {
            certificate_der: certificate.der().clone(),
            certificate_pem: certificate.pem(),
            issuer: Issuer::new(params, key),
        }
    }

    #[must_use]
    pub fn certificate_der(&self) -> &CertificateDer<'static> {
        &self.certificate_der
    }

    #[must_use]
    pub fn certificate_pem(&self) -> &str {
        &self.certificate_pem
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    #[must_use]
    pub fn issue(&self, common_name: &str, subject_alt_name: SanType) -> IssuedCertificate {
        let SignedLeaf { certificate, key } = self.signed_leaf(common_name, subject_alt_name);

        IssuedCertificate {
            certificate_der: certificate.der().clone(),
            private_key: PrivateKeyDer::Pkcs8(key.serialize_der().into()),
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    #[must_use]
    pub fn issue_pem(&self, common_name: &str, subject_alt_name: SanType) -> IssuedPemCertificate {
        let SignedLeaf { certificate, key } = self.signed_leaf(common_name, subject_alt_name);

        IssuedPemCertificate {
            certificate_pem: certificate.pem(),
            private_key_pem: key.serialize_pem(),
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

    fn signed_leaf(&self, common_name: &str, subject_alt_name: SanType) -> SignedLeaf {
        let key = KeyPair::generate().expect("the leaf key pair generates");
        let mut params = CertificateParams::default();

        params.distinguished_name = named(common_name);
        params.subject_alt_names = vec![subject_alt_name];

        SignedLeaf {
            certificate: params
                .signed_by(&key, &self.issuer)
                .expect("the leaf certificate is signed by the CA"),
            key,
        }
    }
}

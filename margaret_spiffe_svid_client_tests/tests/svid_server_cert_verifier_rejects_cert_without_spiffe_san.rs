use rustls::CertificateError;
use rustls::Error;
use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;

use margaret_peer_identity::peer_identity_error::PeerIdentityError;
use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_dns_only_der::LEAF_DNS_ONLY_DER;

#[test]
fn rejects_cert_without_spiffe_san() {
    install_crypto_provider();

    let verifier =
        SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org".to_string())
            .unwrap();

    let result = verifier.verify_server_cert(
        &CertificateDer::from(LEAF_DNS_ONLY_DER.to_vec()),
        &[],
        &ServerName::try_from("ignored.example.org").unwrap(),
        &[],
        UnixTime::now(),
    );

    let Err(Error::InvalidCertificate(CertificateError::Other(other))) = result else {
        panic!("a certificate without a spiffe uri san is rejected as invalid");
    };

    assert!(matches!(
        other
            .0
            .downcast_ref::<PeerIdentityError>()
            .expect("the rejection carries the peer identity source"),
        PeerIdentityError::MissingUri
    ));
}

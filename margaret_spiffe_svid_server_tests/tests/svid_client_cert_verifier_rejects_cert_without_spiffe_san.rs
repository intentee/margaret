use rustls::CertificateError;
use rustls::Error;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_peer_identity::peer_identity_error::PeerIdentityError;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_dns_only_der::LEAF_DNS_ONLY_DER;

#[test]
fn rejects_cert_without_spiffe_san() {
    install_crypto_provider();

    let verifier =
        SvidClientCertVerifier::new(build_root_cert_store_with_ca(), "example.org")
            .expect("the verifier is built");

    let result = verifier.verify_client_cert(
        &CertificateDer::from(LEAF_DNS_ONLY_DER.to_vec()),
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

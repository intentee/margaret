use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;

#[test]
fn accepts_matching_trust_domain() {
    install_crypto_provider();

    let verifier =
        SvidClientCertVerifier::new(build_root_cert_store_with_ca(), "example.org").unwrap();

    let result = verifier.verify_client_cert(
        &CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER.to_vec()),
        &[],
        UnixTime::now(),
    );

    assert!(result.is_ok());
}

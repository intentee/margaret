use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;

use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;

#[test]
fn accepts_matching_trust_domain() {
    install_crypto_provider();

    let verifier =
        SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org").unwrap();

    let result = verifier.verify_server_cert(
        &CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec()),
        &[],
        &ServerName::try_from("ignored.example.org").unwrap(),
        &[],
        UnixTime::now(),
    );

    assert!(result.is_ok());
}

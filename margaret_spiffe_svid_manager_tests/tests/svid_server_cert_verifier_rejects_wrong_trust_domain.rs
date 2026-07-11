use margaret_spiffe_svid_manager::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_manager_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_OTHER_ORG_SERVER_DER;
use rustls::CertificateError;
use rustls::Error;
use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;

#[test]
fn rejects_wrong_trust_domain() {
    install_crypto_provider();

    let verifier =
        SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org".to_string())
            .unwrap();

    let result = verifier.verify_server_cert(
        &CertificateDer::from(LEAF_SPIFFE_OTHER_ORG_SERVER_DER.to_vec()),
        &[],
        &ServerName::try_from("ignored.example.org").unwrap(),
        &[],
        UnixTime::now(),
    );

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
    );
}

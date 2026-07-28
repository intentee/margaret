use margaret_spiffe_svid_client::svid_error::SvidError;
use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn rejects_an_invalid_trust_domain() {
    install_crypto_provider();

    let result =
        SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "EXAMPLE.ORG".to_string());

    assert!(matches!(result, Err(SvidError::TrustDomain { .. })));
}

use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_server::svid_error::SvidError;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn rejects_an_invalid_trust_domain() {
    install_crypto_provider();

    let result =
        SvidClientCertVerifier::new(build_root_cert_store_with_ca(), "EXAMPLE.ORG");

    assert!(matches!(result, Err(SvidError::TrustDomain { .. })));
}

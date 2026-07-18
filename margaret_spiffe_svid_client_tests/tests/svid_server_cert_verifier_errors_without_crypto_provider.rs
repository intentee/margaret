use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn new_errors_when_default_crypto_provider_is_not_installed() {
    let result =
        SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org".to_string());

    assert!(result.is_err());
}

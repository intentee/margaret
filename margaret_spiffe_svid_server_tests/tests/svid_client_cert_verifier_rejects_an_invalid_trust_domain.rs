use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_server::svid_error::SvidError;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn rejects_an_invalid_trust_domain() {
    let result = SvidClientCertVerifier::new(
        build_root_cert_store_with_ca(),
        "EXAMPLE.ORG",
        svid_crypto_provider(),
    );

    assert!(matches!(result, Err(SvidError::TrustDomain { .. })));
}

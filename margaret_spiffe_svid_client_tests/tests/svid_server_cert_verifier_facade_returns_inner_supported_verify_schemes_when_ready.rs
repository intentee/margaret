use std::sync::Arc;

use rustls::client::danger::ServerCertVerifier as _;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn returns_inner_supported_verify_schemes_when_ready() {
    let facade = SvidServerCertVerifierFacade::default();
    facade.update_internal_verifier(Arc::new(
        SvidServerCertVerifier::new(
            build_root_cert_store_with_ca(),
            "example.org",
            svid_crypto_provider(),
        )
        .unwrap(),
    ));

    assert!(!facade.supported_verify_schemes().is_empty());
}

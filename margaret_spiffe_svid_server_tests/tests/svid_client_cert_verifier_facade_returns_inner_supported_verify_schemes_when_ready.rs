use std::sync::Arc;

use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_server::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn returns_inner_supported_verify_schemes_when_ready() {
    let facade = SvidClientCertVerifierFacade::default();
    facade.update_internal_verifier(Arc::new(
        SvidClientCertVerifier::new(
            build_root_cert_store_with_ca(),
            "example.org",
            svid_crypto_provider(),
        )
        .unwrap(),
    ));

    assert!(!facade.supported_verify_schemes().is_empty());
}

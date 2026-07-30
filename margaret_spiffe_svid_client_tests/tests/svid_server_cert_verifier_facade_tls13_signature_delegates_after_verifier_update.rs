use std::sync::Arc;

use rustls::SignatureScheme;
use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_tests::build_digitally_signed_struct::build_digitally_signed_struct;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;

#[test]
fn tls13_signature_delegates_after_verifier_update() {
    install_crypto_provider();

    let facade = SvidServerCertVerifierFacade::default();
    facade.update_internal_verifier(Arc::new(
        SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org")
            .unwrap(),
    ));
    let dss = build_digitally_signed_struct(SignatureScheme::ECDSA_NISTP256_SHA256, &[0u8; 64]);

    let result = facade.verify_tls13_signature(
        b"any message",
        &CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec()),
        &dss,
    );

    assert!(result.is_err());
}

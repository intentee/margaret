use std::sync::Arc;

use rustls::SignatureScheme;
use rustls::pki_types::CertificateDer;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_server::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;
use margaret_spiffe_svid_tests::build_digitally_signed_struct::build_digitally_signed_struct;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;

#[test]
fn tls13_signature_delegates_after_verifier_update() {
    let facade = SvidClientCertVerifierFacade::default();
    facade.update_internal_verifier(Arc::new(
        SvidClientCertVerifier::new(
            build_root_cert_store_with_ca(),
            "example.org",
            svid_crypto_provider(),
        )
        .unwrap(),
    ));
    let dss = build_digitally_signed_struct(SignatureScheme::ECDSA_NISTP256_SHA256, &[0u8; 64]);

    let result = facade.verify_tls13_signature(
        b"any message",
        &CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER.to_vec()),
        &dss,
    );

    assert!(result.is_err());
}

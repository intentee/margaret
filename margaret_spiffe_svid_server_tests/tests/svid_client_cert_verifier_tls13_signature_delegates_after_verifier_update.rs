use rustls::SignatureScheme;
use rustls::pki_types::CertificateDer;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_digitally_signed_struct::build_digitally_signed_struct;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;

#[test]
fn tls13_signature_delegates_after_verifier_update() {
    install_crypto_provider();

    let verifier =
        SvidClientCertVerifier::new(build_root_cert_store_with_ca(), "example.org".to_string())
            .unwrap();
    let dss = build_digitally_signed_struct(SignatureScheme::ECDSA_NISTP256_SHA256, vec![0u8; 64]);

    let result = verifier.verify_tls13_signature(
        b"any message",
        &CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER.to_vec()),
        &dss,
    );

    assert!(result.is_err());
}

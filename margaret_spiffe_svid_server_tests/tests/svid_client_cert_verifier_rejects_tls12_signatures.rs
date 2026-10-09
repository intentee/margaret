use rustls::Error;
use rustls::SignatureScheme;
use rustls::pki_types::CertificateDer;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_digitally_signed_struct::build_digitally_signed_struct;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn rejects_tls12_signatures() {
    let verifier = SvidClientCertVerifier::new(
        build_root_cert_store_with_ca(),
        "example.org",
        svid_crypto_provider(),
    )
    .unwrap();
    let dss = build_digitally_signed_struct(SignatureScheme::RSA_PSS_SHA256, &[0u8; 256]);

    let result =
        verifier.verify_tls12_signature(b"any message", &CertificateDer::from(vec![0u8]), &dss);

    assert_eq!(
        result.unwrap_err(),
        Error::General("TLS 1.2 is not supported".to_string())
    );
}

use rustls::Error;
use rustls::SignatureScheme;
use rustls::pki_types::CertificateDer;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;
use margaret_spiffe_svid_tests::build_digitally_signed_struct::build_digitally_signed_struct;

#[test]
fn tls13_signature_returns_error_when_not_ready() {
    let facade = SvidClientCertVerifierFacade::default();
    let dss = build_digitally_signed_struct(SignatureScheme::ECDSA_NISTP256_SHA256, vec![0u8; 64]);

    let result =
        facade.verify_tls13_signature(b"any message", &CertificateDer::from(vec![0u8]), &dss);

    assert_eq!(
        result.unwrap_err(),
        Error::General("Server is not ready yet (SVID client cert)".to_string())
    );
}

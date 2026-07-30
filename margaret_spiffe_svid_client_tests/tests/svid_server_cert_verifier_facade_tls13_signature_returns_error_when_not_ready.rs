use rustls::Error;
use rustls::SignatureScheme;
use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_tests::build_digitally_signed_struct::build_digitally_signed_struct;

#[test]
fn tls13_signature_returns_error_when_not_ready() {
    let facade = SvidServerCertVerifierFacade::default();
    let dss = build_digitally_signed_struct(SignatureScheme::ECDSA_NISTP256_SHA256, &[0u8; 64]);

    let result =
        facade.verify_tls13_signature(b"any message", &CertificateDer::from(vec![0u8]), &dss);

    assert_eq!(
        result.unwrap_err(),
        Error::General("Client is not ready yet (SVID server cert)".to_string())
    );
}

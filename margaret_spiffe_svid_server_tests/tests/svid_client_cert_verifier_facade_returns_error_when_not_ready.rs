use rustls::Error;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;

#[test]
fn returns_error_when_not_ready() {
    let facade = SvidClientCertVerifierFacade::default();
    let empty_cert = CertificateDer::from(vec![0u8]);

    let result = facade.verify_client_cert(&empty_cert, &[], UnixTime::now());

    assert_eq!(
        result.unwrap_err(),
        Error::General("Server is not ready yet (SVID client cert)".to_string())
    );
}

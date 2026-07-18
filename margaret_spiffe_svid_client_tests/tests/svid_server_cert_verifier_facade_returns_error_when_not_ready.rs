use rustls::Error;
use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;

use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;

#[test]
fn returns_error_when_not_ready() {
    let facade = SvidServerCertVerifierFacade::default();
    let empty_cert = CertificateDer::from(vec![0u8]);

    let result = facade.verify_server_cert(
        &empty_cert,
        &[],
        &ServerName::try_from("ignored.example.org").unwrap(),
        &[],
        UnixTime::now(),
    );

    assert_eq!(
        result.unwrap_err(),
        Error::General("Client is not ready yet (SVID server cert)".to_string())
    );
}

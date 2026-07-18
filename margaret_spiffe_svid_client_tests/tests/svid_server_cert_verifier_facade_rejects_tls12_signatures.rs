use rustls::Error;
use rustls::SignatureScheme;
use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_tests::build_digitally_signed_struct::build_digitally_signed_struct;

#[test]
fn rejects_tls12_signatures() {
    let facade = SvidServerCertVerifierFacade::default();
    let dss = build_digitally_signed_struct(SignatureScheme::RSA_PSS_SHA256, vec![0u8; 256]);

    let result =
        facade.verify_tls12_signature(b"any message", &CertificateDer::from(vec![0u8]), &dss);

    assert_eq!(
        result.unwrap_err(),
        Error::General("TLS 1.2 is not supported".to_string())
    );
}

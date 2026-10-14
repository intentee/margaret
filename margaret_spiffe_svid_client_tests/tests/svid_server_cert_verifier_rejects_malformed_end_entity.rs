use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_client_tests::rejected_by_webpki::rejected_by_webpki;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn rejects_a_non_der_server_certificate_as_badly_encoded_so_the_handshake_fails() {
    let verifier = SvidServerCertVerifier::new(
        build_root_cert_store_with_ca(),
        "example.org",
        svid_crypto_provider(),
    )
    .unwrap();

    let result = verifier.verify_server_cert(
        &CertificateDer::from(vec![0u8; 4]),
        &[],
        &ServerName::try_from("ignored.example.org").unwrap(),
        &[],
        UnixTime::now(),
    );

    assert!(rejected_by_webpki(
        &result.unwrap_err(),
        &webpki::Error::TrailingData(webpki::DerTypeId::SignedData)
    ));
}

use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_untrusted_der::LEAF_SPIFFE_EXAMPLE_ORG_UNTRUSTED_DER;

#[test]
fn rejects_a_client_svid_signed_by_an_untrusted_ca_so_forged_identities_are_refused() {
    let verifier = SvidClientCertVerifier::new(
        build_root_cert_store_with_ca(),
        "example.org",
        svid_crypto_provider(),
    )
    .unwrap();

    let result = verifier.verify_client_cert(
        &CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_UNTRUSTED_DER.to_vec()),
        &[],
        UnixTime::now(),
    );

    assert!(result.is_err());
}

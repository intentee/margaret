use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid_manager::parse_end_entity_cert::parse_end_entity_cert;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;

#[test]
fn parses_valid_cert() {
    let cert = CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec());

    assert!(parse_end_entity_cert(&cert).is_ok());
}

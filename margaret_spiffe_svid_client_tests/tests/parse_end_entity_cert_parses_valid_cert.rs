use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid_client::parse_end_entity_cert::parse_end_entity_cert;
use margaret_spiffe_svid_tests::constants::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;

#[test]
fn parses_a_well_formed_spiffe_leaf_certificate_so_its_identity_can_be_checked() {
    let cert = CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec());

    assert!(parse_end_entity_cert(&cert).is_ok());
}

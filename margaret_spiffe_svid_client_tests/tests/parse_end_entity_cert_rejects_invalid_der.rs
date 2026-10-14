use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid_client::parse_end_entity_cert::parse_end_entity_cert;
use margaret_spiffe_svid_client_tests::rejected_by_webpki::rejected_by_webpki;

#[test]
fn rejects_bytes_that_are_not_der_as_badly_encoded_so_no_identity_is_read_from_them() {
    let cert = CertificateDer::from(vec![0u8; 4]);

    let error = parse_end_entity_cert(&cert).err().unwrap();

    assert!(rejected_by_webpki(
        &error,
        &webpki::Error::TrailingData(webpki::DerTypeId::SignedData)
    ));
}

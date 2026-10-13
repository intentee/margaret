use rcgen::SanType;
use rcgen::string::Ia5String;

use margaret_peer_identity::spiffe_id_extraction::SpiffeIdExtraction;
use margaret_peer_identity::spiffe_id_from_cert::spiffe_id_from_cert;
use margaret_peer_identity::spiffe_id_rejection::SpiffeIdRejection;
use margaret_peer_identity_tests::self_signed_certificate_der::self_signed_certificate_der;

#[test]
fn spiffe_id_from_cert_reports_a_missing_uri() {
    let certificate_der = self_signed_certificate_der(vec![SanType::DnsName(
        Ia5String::try_from("example.org").expect("the DNS name is a valid IA5 string"),
    )]);

    assert!(matches!(
        spiffe_id_from_cert(&certificate_der),
        SpiffeIdExtraction::Rejected(SpiffeIdRejection::MissingUri)
    ));
}

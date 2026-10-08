use rcgen::SanType;
use rcgen::string::Ia5String;

use margaret_peer_identity::spiffe_id_extraction::SpiffeIdExtraction;
use margaret_peer_identity::spiffe_id_from_cert::spiffe_id_from_cert;
use margaret_peer_identity::spiffe_id_rejection::SpiffeIdRejection;
use margaret_peer_identity_tests::self_signed_certificate_der::self_signed_certificate_der;

#[test]
fn spiffe_id_from_cert_rejects_a_non_spiffe_uri() {
    let certificate_der = self_signed_certificate_der(vec![SanType::URI(
        Ia5String::try_from("https://example.org/workload").expect("the URI is a valid IA5 string"),
    )]);

    assert!(matches!(
        spiffe_id_from_cert(&certificate_der),
        SpiffeIdExtraction::Rejected(SpiffeIdRejection::SpiffeId { .. })
    ));
}

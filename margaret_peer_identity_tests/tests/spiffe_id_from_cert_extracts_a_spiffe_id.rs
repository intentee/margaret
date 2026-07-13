use margaret_peer_identity::spiffe_id_from_cert::spiffe_id_from_cert;
use margaret_peer_identity_tests::self_signed_certificate_der::self_signed_certificate_der;
use rcgen::SanType;
use rcgen::string::Ia5String;

#[test]
fn spiffe_id_from_cert_extracts_a_spiffe_id() {
    let certificate_der = self_signed_certificate_der(vec![SanType::URI(
        Ia5String::try_from("spiffe://example.org/workload").expect("the URI is a valid IA5 string"),
    )]);

    let spiffe_id = spiffe_id_from_cert(&certificate_der).expect("a SPIFFE ID is extracted");

    assert_eq!(spiffe_id.trust_domain().to_string(), "example.org");
    assert_eq!(spiffe_id.path(), "/workload");
}

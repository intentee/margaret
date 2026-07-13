use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_peer_identity_tests::self_signed_certificate_der::self_signed_certificate_der;
use rcgen::SanType;
use rcgen::string::Ia5String;

#[test]
fn peer_identity_is_verified_for_a_spiffe_certificate() {
    let certificate_der = self_signed_certificate_der(vec![SanType::URI(
        Ia5String::try_from("spiffe://example.org/workload").expect("the URI is a valid IA5 string"),
    )]);

    let PeerIdentity::Verified { spiffe_id } =
        PeerIdentity::from_peer_certificate(Some(&certificate_der))
    else {
        panic!("expected a verified peer identity");
    };

    assert_eq!(spiffe_id.trust_domain().to_string(), "example.org");
    assert_eq!(spiffe_id.path(), "/workload");
}

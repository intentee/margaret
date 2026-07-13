use margaret_peer_identity::peer_identity::PeerIdentity;

#[test]
fn peer_identity_is_unidentified_for_a_non_spiffe_certificate() {
    assert!(matches!(
        PeerIdentity::from_peer_certificate(Some(b"this is not a certificate")),
        PeerIdentity::Unidentified
    ));
}

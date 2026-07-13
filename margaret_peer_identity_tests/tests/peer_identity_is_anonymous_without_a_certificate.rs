use margaret_peer_identity::peer_identity::PeerIdentity;

#[test]
fn peer_identity_is_anonymous_without_a_certificate() {
    assert!(matches!(
        PeerIdentity::from_peer_certificate(None),
        PeerIdentity::Anonymous
    ));
}

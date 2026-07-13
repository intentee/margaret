use margaret_peer_identity::peer_identity_error::PeerIdentityError;
use margaret_peer_identity::spiffe_id_from_cert::spiffe_id_from_cert;

#[test]
fn spiffe_id_from_cert_rejects_malformed_der() {
    assert!(matches!(
        spiffe_id_from_cert(b"this is not a certificate"),
        Err(PeerIdentityError::CertificateEncoding { .. })
    ));
}

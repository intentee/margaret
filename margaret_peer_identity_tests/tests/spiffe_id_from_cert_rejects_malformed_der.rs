use margaret_peer_identity::spiffe_id_extraction::SpiffeIdExtraction;
use margaret_peer_identity::spiffe_id_from_cert::spiffe_id_from_cert;
use margaret_peer_identity::spiffe_id_rejection::SpiffeIdRejection;

#[test]
fn spiffe_id_from_cert_rejects_malformed_der() {
    assert!(matches!(
        spiffe_id_from_cert(b"this is not a certificate"),
        SpiffeIdExtraction::Rejected(SpiffeIdRejection::CertificateEncoding { .. })
    ));
}

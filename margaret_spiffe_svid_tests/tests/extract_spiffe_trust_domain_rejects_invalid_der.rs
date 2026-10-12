use margaret_peer_identity::spiffe_id_rejection::SpiffeIdRejection;
use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_tests::peer_identity_rejection::peer_identity_rejection;

#[test]
fn reports_an_undecodable_certificate_as_an_encoding_rejection_so_the_handshake_fails() {
    let error = extract_spiffe_trust_domain(&[0u8; 4])
        .expect_err("the certificate does not carry a usable spiffe id");

    assert!(matches!(
        peer_identity_rejection(&error),
        Some(SpiffeIdRejection::CertificateEncoding { .. })
    ));
}

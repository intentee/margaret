use margaret_peer_identity::peer_identity_error::PeerIdentityError;
use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_tests::peer_identity_rejection::peer_identity_rejection;

#[test]
fn rejects_invalid_der() {
    let error = extract_spiffe_trust_domain(&[0u8; 4])
        .expect_err("the certificate does not carry a usable spiffe id");

    assert!(matches!(
        peer_identity_rejection(&error),
        Some(PeerIdentityError::CertificateEncoding { .. })
    ));
}

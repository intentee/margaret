use margaret_peer_identity::peer_identity_error::PeerIdentityError;
use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_tests::leaf_dns_only_der::LEAF_DNS_ONLY_DER;
use margaret_spiffe_svid_tests::peer_identity_rejection::peer_identity_rejection;

#[test]
fn rejects_cert_without_spiffe_san() {
    let error = extract_spiffe_trust_domain(LEAF_DNS_ONLY_DER)
        .expect_err("the certificate does not carry a usable spiffe id");

    assert!(matches!(
        peer_identity_rejection(&error),
        Some(PeerIdentityError::MissingUri)
    ));
}

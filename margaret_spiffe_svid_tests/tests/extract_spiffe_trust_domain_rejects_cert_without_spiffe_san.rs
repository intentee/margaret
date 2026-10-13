use margaret_peer_identity::spiffe_id_rejection::SpiffeIdRejection;
use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_tests::constants::LEAF_DNS_ONLY_DER;
use margaret_spiffe_svid_tests::peer_identity_rejection::peer_identity_rejection;

#[test]
fn rejects_a_cert_without_a_uri_san_since_only_a_spiffe_uri_names_a_trust_domain() {
    let error = extract_spiffe_trust_domain(LEAF_DNS_ONLY_DER)
        .expect_err("the certificate does not carry a usable spiffe id");

    assert!(matches!(
        peer_identity_rejection(&error),
        Some(SpiffeIdRejection::MissingUri)
    ));
}

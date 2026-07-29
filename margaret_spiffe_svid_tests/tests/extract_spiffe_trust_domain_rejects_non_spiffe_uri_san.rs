use spiffe::spiffe_id::SpiffeIdError;

use margaret_peer_identity::peer_identity_error::PeerIdentityError;
use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_tests::leaf_https_uri_der::LEAF_HTTPS_URI_DER;
use margaret_spiffe_svid_tests::peer_identity_rejection::peer_identity_rejection;

#[test]
fn rejects_non_spiffe_uri_san() {
    let error = extract_spiffe_trust_domain(LEAF_HTTPS_URI_DER)
        .expect_err("the certificate does not carry a usable spiffe id");

    assert!(matches!(
        peer_identity_rejection(&error),
        Some(PeerIdentityError::SpiffeId {
            source: SpiffeIdError::WrongScheme
        })
    ));
}

use spiffe::spiffe_id::SpiffeId;

use margaret_peer_identity::peer_identity::PeerIdentity;

/// # Panics
///
/// Panics when the fixture SPIFFE ID does not parse.
#[must_use]
pub fn fixture_workload() -> PeerIdentity {
    PeerIdentity::Verified {
        spiffe_id: SpiffeId::new("spiffe://example.org/relying").expect("the SPIFFE ID parses"),
    }
}

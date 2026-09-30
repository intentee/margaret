use http::Method;
use spiffe::spiffe_id::SpiffeId;

use margaret_http::require_peer_spiffe_id::require_peer_spiffe_id;
use margaret_http::requirement::Requirement;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_peer_identity::peer_identity::PeerIdentity;

#[test]
fn requires_the_spiffe_id_of_a_verified_peer() {
    let spiffe_id = SpiffeId::new("spiffe://example.org/workload").expect("the SPIFFE ID parses");
    let mut fixture = FixtureRequest::new(Method::GET, "/");

    fixture.peer_identity = PeerIdentity::Verified {
        spiffe_id: spiffe_id.clone(),
    };

    let request = fixture.into_request();

    assert!(matches!(
        require_peer_spiffe_id(&request),
        Requirement::Met(verified) if verified == &spiffe_id
    ));
}

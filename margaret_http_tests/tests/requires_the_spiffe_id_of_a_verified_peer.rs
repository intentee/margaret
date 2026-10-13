use http::Method;
use spiffe::spiffe_id::SpiffeId;

use margaret_http::requirement::Requirement;
use margaret_http::with_verified_peer::with_verified_peer;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_http_tests::met_verified_peer::met_verified_peer;
use margaret_peer_identity::peer_identity::PeerIdentity;

#[tokio::test]
async fn requires_the_spiffe_id_of_a_verified_peer() {
    let spiffe_id = SpiffeId::new("spiffe://example.org/workload").expect("the SPIFFE ID parses");
    let mut fixture = FixtureRequest::new(Method::GET, "/");

    fixture.peer_identity = PeerIdentity::Verified {
        spiffe_id: spiffe_id.clone(),
    };

    let request = fixture.into_request();

    assert!(matches!(
        with_verified_peer(&request, Requirement::Unmet, met_verified_peer)
        .await,
        Requirement::Met(verified) if verified == spiffe_id
    ));
}

use http::Method;

use margaret_http::require_peer_spiffe_id::require_peer_spiffe_id;
use margaret_http::requirement::Requirement;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_peer_identity::peer_identity::PeerIdentity;

#[test]
fn forbids_an_anonymous_peer() {
    let mut fixture = FixtureRequest::new(Method::GET, "/");

    fixture.peer_identity = PeerIdentity::Anonymous;

    let request = fixture.into_request();

    assert!(matches!(
        require_peer_spiffe_id(&request),
        Requirement::Unmet(ResponseContinuation::Done(response)) if response.status() == 403
    ));
}

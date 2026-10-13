use http::Method;

use margaret_http::requirement::Requirement;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::with_verified_peer::with_verified_peer;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_http_tests::met_verified_peer::met_verified_peer;
use margaret_peer_identity::peer_identity::PeerIdentity;

#[tokio::test]
async fn forbids_a_peer_whose_certificate_yields_no_spiffe_id_since_it_cannot_be_identified() {
    let mut fixture = FixtureRequest::new(Method::GET, "/");

    fixture.peer_identity = PeerIdentity::Unidentified;

    let request = fixture.into_request();

    assert!(matches!(
        with_verified_peer(&request, Requirement::Unmet, met_verified_peer)
        .await,
        Requirement::Unmet(ResponseContinuation::Done(response)) if response.status() == 403
    ));
}

use http::Method;

use margaret_http::requirement::Requirement;
use margaret_http::with_verified_peer::with_verified_peer;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_http_tests::met_verified_peer::met_verified_peer;
use margaret_peer_identity::peer_identity::PeerIdentity;

#[tokio::test]
async fn forbids_a_peer_that_presented_no_client_certificate_since_it_has_no_spiffe_id() {
    let mut fixture = FixtureRequest::new(Method::GET, "/");

    fixture.peer_identity = PeerIdentity::Anonymous;

    let request = fixture.into_request();

    assert!(matches!(
        with_verified_peer(&request, Requirement::Unmet, met_verified_peer)
        .await,
        Requirement::Unmet(response) if response.status() == 403
    ));
}

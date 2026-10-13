use http::HeaderValue;
use http::Method;
use http::header::CONTENT_TYPE;

use margaret_http::request::Request;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_peer_identity::peer_identity::PeerIdentity;

#[must_use]
pub fn form_request(peer_identity: PeerIdentity) -> Request {
    let mut fixture = FixtureRequest::new(Method::POST, "/sessions/refresh");

    fixture.headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    fixture.peer_identity = peer_identity;
    fixture.into_request()
}

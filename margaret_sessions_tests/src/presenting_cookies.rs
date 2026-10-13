use cookie::Cookie;
use http::Method;

use margaret_http::request::Request;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_peer_identity::peer_identity::PeerIdentity;

#[must_use]
pub fn presenting_cookies(
    method: Method,
    cookies: &[Cookie<'_>],
    peer_identity: PeerIdentity,
) -> Request {
    let mut fixture = FixtureRequest::new(method, "/").presenting_cookies(cookies);

    fixture.peer_identity = peer_identity;
    fixture.into_request()
}

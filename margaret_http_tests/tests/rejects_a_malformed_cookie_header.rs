use http::HeaderValue;
use http::Method;
use http::header::COOKIE;

use margaret_http::request_rejection::RequestRejection;
use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn rejects_a_malformed_cookie_header() {
    let mut fixture = FixtureRequest::new(Method::GET, "/");

    fixture
        .headers
        .insert(COOKIE, HeaderValue::from_static("=nameless"));

    assert!(matches!(
        fixture.into_result(),
        Err(RequestRejection::MalformedCookie { .. })
    ));
}

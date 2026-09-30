use http::HeaderValue;
use http::Method;
use http::header::COOKIE;

use margaret_http::request_rejection::RequestRejection;
use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn rejects_a_repeated_cookie_name() {
    let mut fixture = FixtureRequest::new(Method::GET, "/");

    fixture.headers.insert(
        COOKIE,
        HeaderValue::from_static("session=abc; session=stolen"),
    );

    assert!(matches!(
        fixture.into_result(),
        Err(RequestRejection::DuplicateCookie { name }) if name == "session"
    ));
}

use cookie::Cookie;
use http::HeaderValue;
use http::Method;
use http::header::CONTENT_TYPE;

use margaret_http::request::Request;
use margaret_http_tests::fixture_request::FixtureRequest;

#[must_use]
pub fn form_post_request(cookies: &[Cookie<'_>]) -> Request {
    let mut fixture = FixtureRequest::new(Method::POST, "/").presenting_cookies(cookies);

    fixture.headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    fixture.into_request()
}

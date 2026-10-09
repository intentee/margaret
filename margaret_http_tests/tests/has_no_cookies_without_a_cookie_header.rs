use http::Method;

use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn has_no_cookies_without_a_cookie_header() {
    assert!(
        FixtureRequest::new(Method::GET, "/")
            .into_request()
            .inputs
            .cookies
            .is_empty()
    );
}

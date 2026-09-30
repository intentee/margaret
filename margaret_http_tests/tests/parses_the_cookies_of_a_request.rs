use http::HeaderValue;
use http::Method;
use http::header::COOKIE;

use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn parses_the_cookies_of_a_request() {
    let mut fixture = FixtureRequest::new(Method::GET, "/");

    fixture
        .headers
        .insert(COOKIE, HeaderValue::from_static("session=abc; theme=dark"));

    let request = fixture.into_request();

    assert_eq!(
        request.inputs.cookies.get("session").map(String::as_str),
        Some("abc")
    );
    assert_eq!(
        request.inputs.cookies.get("theme").map(String::as_str),
        Some("dark")
    );
}

use std::collections::BTreeMap;

use http::HeaderValue;
use http::Method;
use http::header::COOKIE;

use margaret_http::request::Request;
use margaret_http_tests::fixture_request::FixtureRequest;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn callback_request(cookie: &str, parameters: &BTreeMap<&str, &str>) -> Request {
    let query = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(parameters)
        .finish();
    let mut fixture = FixtureRequest::new(Method::GET, &format!("/callback?{query}"));

    fixture.headers.insert(
        COOKIE,
        HeaderValue::from_str(cookie).expect("the cookie is a header value"),
    );

    fixture.into_request()
}

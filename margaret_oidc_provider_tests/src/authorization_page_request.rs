use cookie::Cookie;
use http::Method;
use serde_json::Value;

use margaret_http::request::Request;
use margaret_http_tests::fixture_request::FixtureRequest;

use crate::form_encoded::form_encoded;

#[must_use]
pub fn authorization_page_request(parameters: &Value, cookies: &[Cookie<'_>]) -> Request {
    FixtureRequest::new(
        Method::GET,
        &format!("/authorize?{}", form_encoded(parameters)),
    )
    .presenting_cookies(cookies)
    .into_request()
}

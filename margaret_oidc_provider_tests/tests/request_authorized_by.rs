use http::HeaderValue;
use http::Method;
use http::header::AUTHORIZATION;

use margaret_http::request::Request;
use margaret_http_tests::fixture_request::FixtureRequest;

pub fn request_authorized_by(authorization: &str) -> Request {
    let mut fixture = FixtureRequest::new(Method::POST, "/");

    fixture.headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(authorization).expect("the authorization is a header value"),
    );

    fixture.into_request()
}
